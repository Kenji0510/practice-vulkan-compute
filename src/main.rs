use anyhow::Result;
use practice_vulkano::check_device::check_devices;

fn main() -> Result<()> {
    check_devices()?;

    Ok(())
}
