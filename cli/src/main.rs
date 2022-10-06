use anyhow::Result;
use v4l2::Device;

fn main() -> Result<()> {
    let dev = Device::open("/dev/video0")?;

    let caps = dev.capabilities()?;

    println!("Capabilities: {}", caps);

    println!("Controls:");
    for ctrl in dev.controls() {
        let ctrl = ctrl?;
        println!("  {}", ctrl);

        if let Some(items) = dev.control_items(&ctrl) {
            for item in items {
                let item = item?;
                println!("    {}", item);
            }
        }
    }

    Ok(())
}
