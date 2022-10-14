use anyhow::Result;
use v4l2::{types::*, Device};

fn main() -> Result<()> {
    let dev = Device::open("/dev/video2")?;

    let caps = dev.capabilities()?;

    println!("Capabilities: {}", caps);

    println!("Controls:");
    for ctrl in dev.controls(None) {
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

        for size in dev.sizes(fmt.pixel_format()) {
            let size = size?;
            println!("    {}", size);

            for size in size.sizes() {
                println!("      {}", size);
                for interval in dev.intervals(fmt.pixel_format(), size.width(), size.height()) {
                    let interval = interval?;
                    println!("        {}", interval);
                }
            }
        }
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

    let frames = dev.queue::<In, Mmap>(In::Video, 2)?;

    let mut i = 0;
    while let Ok(frame) = frames.next() {
        //for frame in frames {
        //let data = frame?;

        println!("#{} F: {}", i, frame);

        i += 1;
        if i > 30 {
            break;
        }
    }

    contrast.try_mut::<i32>().map(|val| *val = 32);

    dev.set_control(&contrast)?;

    Ok(())
}
