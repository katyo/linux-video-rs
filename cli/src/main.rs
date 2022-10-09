use anyhow::Result;
use v4l2::{types::*, Device};

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

    println!("Formats:");
    for fmt in dev.formats(BufferType::VideoCapture) {
        let fmt = fmt?;
        println!("  {}", fmt);
    }

    let mut fmt = Format::from(BufferType::VideoCapture);
    println!("  {}", fmt);

    dev.get_format(&mut fmt)?;
    println!("  {}", fmt);

    let mut contrast: Value<_> = dev.control(CtrlId::Contrast)?.into();

    println!("contrast control: {}", &*contrast);

    dev.get_control(&mut contrast)?;

    println!("contrast value: {:?}", contrast.try_ref::<i32>());

    let mut contrast = contrast;

    contrast.try_mut::<i32>().map(|val| *val = 42);

    println!("contrast value: {:?}", contrast.try_ref::<i32>());

    dev.set_control(&contrast)?;

    contrast.try_mut::<i32>().map(|val| *val = 32);

    dev.set_control(&contrast)?;

    Ok(())
}
