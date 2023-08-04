use linux_video::{types::*, Device};

fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0")?;

    // Get current format
    let fmt = dev.format(BufferType::VideoCapture)?;
    println!("  {fmt}");

    // Get current params
    let prm = dev.param(BufferType::VideoCapture)?;
    println!("  {prm}");

    // Start video capture stream
    let stream = dev.stream::<In, Mmap>(ContentType::Video, 4)?;

    let mut i = 0;
    while let Ok(buffer) = stream.next() {
        println!("#{i} {buffer}");

        i += 1;
        if i > 30 {
            break;
        }
    }

    Ok(())
}
