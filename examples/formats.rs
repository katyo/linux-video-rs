use linux_video::{types::BufferType, Device};

fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0")?;

    let caps = dev.capabilities()?;

    for type_ in BufferType::ALL {
        if type_.is_supported(caps.capabilities()) {
            println!("{} formats:", type_);
            for fmt in dev.formats(type_) {
                let fmt = fmt?;
                println!("  {}", fmt);

                if type_.content().is_video() {
                    for size in dev.sizes(fmt.pixel_format()) {
                        let size = size?;
                        println!("    {}", size);

                        for size in size.sizes() {
                            println!("      {}", size);
                            for interval in
                                dev.intervals(fmt.pixel_format(), size.width(), size.height())
                            {
                                let interval = interval?;
                                println!("        {}", interval);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
