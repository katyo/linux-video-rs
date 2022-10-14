use linux_video::Device;

fn main() -> std::io::Result<()> {
    let devs = Device::list()?;

    for path in devs {
        let path = path?;

        let dev = Device::open(&path)?;

        let caps = dev.capabilities()?;

        println!("path: {}, {}", path.display(), caps);
    }

    Ok(())
}
