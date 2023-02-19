mod args;
use linux_video::{types::*, Device};
use std::io::Result;

fn main() -> Result<()> {
    use args::{Args, Cmd};

    let args: Args = clap::Parser::parse();

    match args.cmd {
        Cmd::List => {
            for path in Device::list()? {
                println!("{}", path?.display());
            }
        }

        Cmd::Info {
            devices,
            mut capabilities,
            mut controls,
            class,
            mut formats,
            r#type,
            mut sizes,
            mut intervals,
            all,
        } => {
            if all {
              capabilities = true;
              controls = true;
              formats = true;
              sizes = true;
              intervals = true;
            }

            for name in devices {
                let device = Device::open(&name)?;

                println!("{name}");

                let caps = device.capabilities()?;

                if capabilities {
                    println!("  Capabilities: {caps}");
                }

                if controls {
                    println!("  Controls:");

                    if class.is_empty() {
                        print_controls(&device, None)?;
                    } else {
                        for class in &class {
                            print_controls(&device, Some(*class))?;
                        }
                    }
                }

                if formats || sizes || intervals {
                    if r#type.is_empty() {
                        for buffer_type in BufferType::ALL {
                            print_formats(&device, &caps, buffer_type, sizes, intervals)?;
                        }
                    } else {
                        for buffer_type in &r#type {
                            print_formats(&device, &caps, *buffer_type, sizes, intervals)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_controls(device: &Device, class: Option<CtrlClass>) -> Result<()> {
    for ctrl in device.controls(class) {
        let ctrl = ctrl?;
        println!("    {ctrl}");

        if let Some(items) = device.control_items(&ctrl) {
            for item in items {
                let item = item?;
                println!("      {item}");
            }
        }
    }

    Ok(())
}

fn print_formats(
    device: &Device,
    caps: &Capability,
    type_: BufferType,
    sizes: bool,
    intervals: bool,
) -> Result<()> {
    if type_.is_supported(caps.capabilities()) {
        println!("  {type_} Formats:");

        for fmt in device.formats(type_) {
            let fmt = fmt?;
            println!("    {fmt}");

            if sizes || intervals {
                for size in device.sizes(fmt.pixel_format()) {
                    let size = size?;
                    println!("      {size}");

                    for size in size.sizes() {
                        println!("        {size}");

                        if intervals {
                            for interval in
                                device.intervals(fmt.pixel_format(), size.width(), size.height())
                            {
                                let interval = interval?;
                                println!("          {interval}");
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
