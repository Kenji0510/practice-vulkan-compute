use std::sync::Arc;

use anyhow::Result;
use vulkano::{VulkanLibrary, device::physical::PhysicalDevice, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::MemoryHeapFlags};

pub fn check_devices() -> Result<()> {
    let library = VulkanLibrary::new()?;

    // portability subset (MoltenVK 等) も列挙したい場合は ENUMERATE_PORTABILITY を付ける。
    // Windows/Linux だけなら flags は Default のままでもOK。
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    )?;

    let devices: Vec<Arc<PhysicalDevice>> = instance.enumerate_physical_devices()?.collect();
    if devices.is_empty() {
        println!("No Vulkan physical device found.");
        return Ok(());
    }

    for (i, dev) in devices.iter().enumerate() {
        let p = dev.properties();
        let f = dev.supported_features();
        let e = dev.supported_extensions();
        let m = dev.memory_properties();

        println!();
        println!("============================================================");
        println!("[{}] {}  ({:?})", i, p.device_name, p.device_type);
        println!("  api_version        : {}", p.api_version);
        println!("  driver_version     : {}", p.driver_version);
        if !p.driver_name.clone().unwrap().is_empty() {
            println!("  driver_name        : {}", p.driver_name.clone().unwrap());
        }
        if !p.driver_info.clone().unwrap().is_empty() {
            println!("  driver_info        : {}", p.driver_info.clone().unwrap());
        }
        println!("  device_id/vendor_id: {}/{}", p.device_id, p.vendor_id);

        // ----------------------------
        // メモリ (VRAM/heap/type)
        // ----------------------------
        println!();
        println!("-- Memory Heaps --");
        let mut device_local_total: u64 = 0;
        for (hi, heap) in m.memory_heaps.iter().enumerate() {
            let is_device_local = heap.flags.intersects(MemoryHeapFlags::DEVICE_LOCAL);
            if is_device_local {
                device_local_total = device_local_total.saturating_add(heap.size);
            }
            println!(
                "  heap[{}]: size={}, flags={:?}{}",
                hi,
                fmt_bytes(heap.size),
                heap.flags,
                if is_device_local { " (DEVICE_LOCAL)" } else { "" }
            );
        }
        println!("  -> total DEVICE_LOCAL (approx VRAM): {}", fmt_bytes(device_local_total));

        println!();
        println!("-- Memory Types --");
        for (ti, ty) in m.memory_types.iter().enumerate() {
            println!(
                "  type[{}]: heap_index={}, property_flags={:?}",
                ti, ty.heap_index, ty.property_flags
            );
        }

        // ----------------------------
        // “一度に扱えるサイズ”系（重要な上限）
        // ----------------------------
        println!();
        println!("-- Key Limits / Sizes --");
        print_opt_devsize("max_buffer_size", p.max_buffer_size);
        println!("  max_storage_buffer_range     : {}", fmt_bytes(p.max_storage_buffer_range as u64));
        println!("  max_uniform_buffer_range     : {}", fmt_bytes(p.max_uniform_buffer_range as u64));
        print_u32("max_push_constants_size", p.max_push_constants_size);
        print_opt_devsize("max_memory_allocation_size", p.max_memory_allocation_size);
        println!("  max_memory_allocation_count  : {}", p.max_memory_allocation_count);

        // ----------------------------
        // __shared 相当（Workgroup shared memory）
        // ----------------------------
        println!();
        println!("-- Compute / Workgroup (\"__shared\"-like) --");
        println!(
            "  max_compute_shared_memory_size : {}",
            fmt_bytes(p.max_compute_shared_memory_size as u64)
        );
        println!(
            "  max_compute_work_group_invocations : {}",
            p.max_compute_work_group_invocations
        );
        println!(
            "  max_compute_work_group_size : {:?} (x,y,z)",
            p.max_compute_work_group_size
        );
        println!(
            "  max_compute_work_group_count: {:?} (x,y,z)",
            p.max_compute_work_group_count
        );

        println!();
        println!("-- Alignment / Atom size (useful for mapping/SSBO) --");
        println!("  min_memory_map_alignment           : {}", p.min_memory_map_alignment);
        println!("  non_coherent_atom_size             : {}", p.non_coherent_atom_size.as_devicesize());
        println!("  min_storage_buffer_offset_alignment: {}", p.min_storage_buffer_offset_alignment.as_devicesize());
        println!("  min_uniform_buffer_offset_alignment: {}", p.min_uniform_buffer_offset_alignment.as_devicesize());

        // ----------------------------
        // atomic / 整数 / uint 相当
        // ----------------------------
        println!();
        println!("-- Shader integer support --");
        println!("  shader_int16 : {}", yn(f.shader_int16));
        println!("  shader_int64 : {}", yn(f.shader_int64));
        println!("  shader_float16: {}", yn(f.shader_float16));
        println!("  shader_float64: {}", yn(f.shader_float64));

        // atomic の“対応範囲”を見る時に役立つ（どのステージで stores/atomics を許すか）
        println!();
        println!("-- Stores & Atomics in graphics stages (scope) --");
        println!("  vertex_pipeline_stores_and_atomics : {}", yn(f.vertex_pipeline_stores_and_atomics));
        println!("  fragment_stores_and_atomics        : {}", yn(f.fragment_stores_and_atomics));

        // 64-bit int atomics（atomicCAS/compare-exchange を 64-bit でやりたい時の目安）
        println!();
        println!("-- 64-bit integer atomics (for atomicCAS on 64-bit) --");
        println!("  shader_buffer_int64_atomics : {}", yn(f.shader_buffer_int64_atomics));
        println!("  shader_shared_int64_atomics : {}", yn(f.shader_shared_int64_atomics));
        println!("  supported extension (hint)  : khr_shader_atomic_int64={}", yn(e.khr_shader_atomic_int64));

        // float atomics（CUDA 的な atomicAdd(float) 相当）
        println!();
        println!("-- Floating-point atomics --");
        println!("  shader_buffer_float32_atomic_add      : {}", yn(f.shader_buffer_float32_atomic_add));
        println!("  shader_buffer_float32_atomic_min_max  : {}", yn(f.shader_buffer_float32_atomic_min_max));
        println!("  shader_buffer_float64_atomic_add      : {}", yn(f.shader_buffer_float64_atomic_add));
        println!("  shader_buffer_float64_atomic_min_max  : {}", yn(f.shader_buffer_float64_atomic_min_max));
        println!("  shader_image_float32_atomic_add       : {}", yn(f.shader_image_float32_atomic_add));
        println!("  shader_image_float32_atomic_min_max   : {}", yn(f.shader_image_float32_atomic_min_max));
        // println!("  shader_image_float64_atomic_add       : {}", yn(f.shader_image_float64_atomic_add));
        // println!("  shader_image_float64_atomic_min_max   : {}", yn(f.shader_image_float64_atomic_min_max));

        // 32-bit の atomicCAS についての注意喚起（feature で出ないことが多い）
        println!();
        println!("-- Notes (atomicCAS) --");
        println!("  * 32-bit integer atomic CAS (compare-exchange) is generally available in Vulkan shaders.");
        println!("    Stage availability can depend on vertex_pipeline_stores_and_atomics / fragment_stores_and_atomics.");
        println!("  * 64-bit atomic CAS needs shader_buffer_int64_atomics / shader_shared_int64_atomics (and related extension).");
    }

    Ok(())
}

fn yn(b: bool) -> &'static str {
    if b { "YES" } else { "NO" }
}

fn fmt_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;
    if b >= GIB {
        format!("{:.2} GiB ({} bytes)", b / GIB, bytes)
    } else if b >= MIB {
        format!("{:.2} MiB ({} bytes)", b / MIB, bytes)
    } else if b >= KIB {
        format!("{:.2} KiB ({} bytes)", b / KIB, bytes)
    } else {
        format!("{} bytes", bytes)
    }
}

fn print_opt_bytes(name: &str, v: Option<u64>) {
    match v {
        Some(x) => println!("  {:28}: {}", name, fmt_bytes(x)),
        None => println!("  {:28}: N/A (extension / newer core not available)", name),
    }
}

fn print_u32(name: &str, v: u32) {
    println!("  {:28}: {}", name, v);
}

// 例: v が DeviceSize 型なら Option<u64> にするところは as u64 を挟む
fn print_opt_devsize(name: &str, v: Option<vulkano::DeviceSize>) {
    match v {
        Some(x) => println!("  {:28}: {}", name, fmt_bytes(x as u64)),
        None => println!("  {:28}: N/A (extension / newer core not available)", name),
    }
}
