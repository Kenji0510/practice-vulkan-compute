# RTX4080 (Ubuntu22)
```bash
============================================================
[0] NVIDIA GeForce RTX 4080  (DiscreteGpu)
  api_version        : 1.4.312
  driver_version     : 2434253120
  driver_name        : NVIDIA
  driver_info        : 580.95.05
  device_id/vendor_id: 9988/4318

-- Memory Heaps --
  heap[0]: size=15.99 GiB (17171480576 bytes), flags=DEVICE_LOCAL (DEVICE_LOCAL)
  heap[1]: size=23.52 GiB (25256804352 bytes), flags=empty()
  -> total DEVICE_LOCAL (approx VRAM): 15.99 GiB (17171480576 bytes)

-- Memory Types --
  type[0]: heap_index=1, property_flags=empty()
  type[1]: heap_index=0, property_flags=DEVICE_LOCAL
  type[2]: heap_index=1, property_flags=HOST_VISIBLE | HOST_COHERENT
  type[3]: heap_index=1, property_flags=HOST_VISIBLE | HOST_COHERENT | HOST_CACHED
  type[4]: heap_index=0, property_flags=DEVICE_LOCAL | HOST_VISIBLE | HOST_COHERENT

-- Key Limits / Sizes --
  max_buffer_size              : 1024.00 GiB (1099511627776 bytes)
  max_storage_buffer_range     : 4.00 GiB (4294967295 bytes)
  max_uniform_buffer_range     : 64.00 KiB (65536 bytes)
  max_push_constants_size      : 256 bytes
  max_memory_allocation_size   : 4.00 GiB (4292870144 bytes)
  max_memory_allocation_count  : 4294967295

-- Compute / Workgroup ("__shared"-like) --
  max_compute_shared_memory_size : 48.00 KiB (49152 bytes)
  max_compute_work_group_invocations : 1024
  max_compute_work_group_size : [1024, 1024, 64] (x,y,z)
  max_compute_work_group_count: [2147483647, 65535, 65535] (x,y,z)

-- Alignment / Atom size (useful for mapping/SSBO) --
  min_memory_map_alignment           : 64
  non_coherent_atom_size             : 64
  min_storage_buffer_offset_alignment: 16
  min_uniform_buffer_offset_alignment: 64

-- Shader integer support --
  shader_int16 : YES
  shader_int64 : YES
  shader_float16: YES
  shader_float64: YES

-- Stores & Atomics in graphics stages (scope) --
  vertex_pipeline_stores_and_atomics : YES
  fragment_stores_and_atomics        : YES

-- 64-bit integer atomics (for atomicCAS on 64-bit) --
  shader_buffer_int64_atomics : YES
  shader_shared_int64_atomics : YES
  supported extension (hint)  : khr_shader_atomic_int64=YES

-- Floating-point atomics --
  shader_buffer_float32_atomic_add      : YES
  shader_buffer_float32_atomic_min_max  : NO
  shader_buffer_float64_atomic_add      : YES
  shader_buffer_float64_atomic_min_max  : NO
  shader_image_float32_atomic_add       : YES
  shader_image_float32_atomic_min_max   : NO

-- Notes (atomicCAS) --
  * 32-bit integer atomic CAS (compare-exchange) is generally available in Vulkan shaders.
    Stage availability can depend on vertex_pipeline_stores_and_atomics / fragment_stores_and_atomics.
  * 64-bit atomic CAS needs shader_buffer_int64_atomics / shader_shared_int64_atomics (and related extension).
```

# RTX3050 (Ubuntu24)
```bash
============================================================
[0] NVIDIA GeForce RTX 3050  (DiscreteGpu)
  api_version        : 1.4.312
  driver_version     : 2434253120
  driver_name        : NVIDIA
  driver_info        : 580.95.05
  device_id/vendor_id: 9604/4318

-- Memory Heaps --
  heap[0]: size=6.00 GiB (6442450944 bytes), flags=DEVICE_LOCAL (DEVICE_LOCAL)
  heap[1]: size=11.70 GiB (12559958016 bytes), flags=empty()
  -> total DEVICE_LOCAL (approx VRAM): 6.00 GiB (6442450944 bytes)

-- Memory Types --
  type[0]: heap_index=1, property_flags=empty()
  type[1]: heap_index=0, property_flags=DEVICE_LOCAL
  type[2]: heap_index=0, property_flags=DEVICE_LOCAL
  type[3]: heap_index=1, property_flags=HOST_VISIBLE | HOST_COHERENT
  type[4]: heap_index=1, property_flags=HOST_VISIBLE | HOST_COHERENT | HOST_CACHED
  type[5]: heap_index=0, property_flags=DEVICE_LOCAL | HOST_VISIBLE | HOST_COHERENT

-- Key Limits / Sizes --
  max_buffer_size              : 1024.00 GiB (1099511627776 bytes)
  max_storage_buffer_range     : 4.00 GiB (4294967295 bytes)
  max_uniform_buffer_range     : 64.00 KiB (65536 bytes)
  max_push_constants_size      : 256 bytes
  max_memory_allocation_size   : 4.00 GiB (4292870144 bytes)
  max_memory_allocation_count  : 4294967295

-- Compute / Workgroup ("__shared"-like) --
  max_compute_shared_memory_size : 48.00 KiB (49152 bytes)
  max_compute_work_group_invocations : 1024
  max_compute_work_group_size : [1024, 1024, 64] (x,y,z)
  max_compute_work_group_count: [2147483647, 65535, 65535] (x,y,z)

-- Alignment / Atom size (useful for mapping/SSBO) --
  min_memory_map_alignment           : 64
  non_coherent_atom_size             : 64
  min_storage_buffer_offset_alignment: 16
  min_uniform_buffer_offset_alignment: 64

-- Shader integer support --
  shader_int16 : YES
  shader_int64 : YES
  shader_float16: YES
  shader_float64: YES

-- Stores & Atomics in graphics stages (scope) --
  vertex_pipeline_stores_and_atomics : YES
  fragment_stores_and_atomics        : YES

-- 64-bit integer atomics (for atomicCAS on 64-bit) --
  shader_buffer_int64_atomics : YES
  shader_shared_int64_atomics : YES
  supported extension (hint)  : khr_shader_atomic_int64=YES

-- Floating-point atomics --
  shader_buffer_float32_atomic_add      : YES
  shader_buffer_float32_atomic_min_max  : NO
  shader_buffer_float64_atomic_add      : YES
  shader_buffer_float64_atomic_min_max  : NO
  shader_image_float32_atomic_add       : YES
  shader_image_float32_atomic_min_max   : NO

-- Notes (atomicCAS) --
  * 32-bit integer atomic CAS (compare-exchange) is generally available in Vulkan shaders.
    Stage availability can depend on vertex_pipeline_stores_and_atomics / fragment_stores_and_atomics.
  * 64-bit atomic CAS needs shader_buffer_int64_atomics / shader_shared_int64_atomics (and related extension).
```

# M4 Pro (20 Cores)
```bash
============================================================
[0] Apple M4 Pro  (IntegratedGpu)
  api_version        : 1.2.296
  driver_version     : 10211
  driver_name        : MoltenVK
  driver_info        : 1.2.11
  device_id/vendor_id: 252117513/4203

-- Memory Heaps --
  heap[0]: size=48.00 GiB (51539607552 bytes), flags=DEVICE_LOCAL (DEVICE_LOCAL)
  -> total DEVICE_LOCAL (approx VRAM): 48.00 GiB (51539607552 bytes)

-- Memory Types --
  type[0]: heap_index=0, property_flags=DEVICE_LOCAL
  type[1]: heap_index=0, property_flags=DEVICE_LOCAL | HOST_VISIBLE | HOST_COHERENT | HOST_CACHED
  type[2]: heap_index=0, property_flags=DEVICE_LOCAL | LAZILY_ALLOCATED

-- Key Limits / Sizes --
  max_buffer_size             : N/A (extension / newer core not available)
  max_storage_buffer_range     : 4.00 GiB (4294967295 bytes)
  max_uniform_buffer_range     : 4.00 GiB (4294967295 bytes)
  max_push_constants_size     : 4096
  max_memory_allocation_size  : 27.00 GiB (28991029248 bytes)
  max_memory_allocation_count  : 1073741824

-- Compute / Workgroup ("__shared"-like) --
  max_compute_shared_memory_size : 32.00 KiB (32768 bytes)
  max_compute_work_group_invocations : 1024
  max_compute_work_group_size : [1024, 1024, 1024] (x,y,z)
  max_compute_work_group_count: [1073741824, 1073741824, 1073741824] (x,y,z)

-- Alignment / Atom size (useful for mapping/SSBO) --
  min_memory_map_alignment           : 64
  non_coherent_atom_size             : 16
  min_storage_buffer_offset_alignment: 16
  min_uniform_buffer_offset_alignment: 16

-- Shader integer support --
  shader_int16 : YES
  shader_int64 : YES
  shader_float16: YES
  shader_float64: NO

-- Stores & Atomics in graphics stages (scope) --
  vertex_pipeline_stores_and_atomics : YES
  fragment_stores_and_atomics        : YES

-- 64-bit integer atomics (for atomicCAS on 64-bit) --
  shader_buffer_int64_atomics : NO
  shader_shared_int64_atomics : NO
  supported extension (hint)  : khr_shader_atomic_int64=NO

-- Floating-point atomics --
  shader_buffer_float32_atomic_add      : YES
  shader_buffer_float32_atomic_min_max  : NO
  shader_buffer_float64_atomic_add      : NO
  shader_buffer_float64_atomic_min_max  : NO
  shader_image_float32_atomic_add       : NO
  shader_image_float32_atomic_min_max   : NO

-- Notes (atomicCAS) --
  * 32-bit integer atomic CAS (compare-exchange) is generally available in Vulkan shaders.
    Stage availability can depend on vertex_pipeline_stores_and_atomics / fragment_stores_and_atomics.
  * 64-bit atomic CAS needs shader_buffer_int64_atomics / shader_shared_int64_atomics (and related extension).
```