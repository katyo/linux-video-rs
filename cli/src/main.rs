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

    let contrast = dev.control(v4l2::types::CtrlId::Contrast)?;

    println!("contrast control: {}", contrast);

    let contrast = dev.control_get(contrast)?;

    println!("contrast value: {:?}", contrast.try_ref::<i32>());

    let mut contrast = contrast;

    contrast.try_mut::<i32>().map(|val| *val = 42);

    println!("contrast value: {:?}", contrast.try_ref::<i32>());

    dev.control_set(&contrast)?;

    contrast.try_mut::<i32>().map(|val| *val = 32);

    dev.control_set(&contrast)?;

    Ok(())
}
