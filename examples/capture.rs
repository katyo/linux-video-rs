use linux_video::{types::*, Device};

fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0")?;

    let mut fmt = Format::from(BufferType::VideoCapture);

    // Get current format
    dev.get_format(&mut fmt)?;
    println!("  {}", fmt);

    // Start video capture stream
    let stream = dev.stream::<In, Mmap>(ContentType::Video, 4)?;

    let mut i = 0;
    while let Ok(buffer) = stream.next() {
        println!("#{} {}", i, buffer);

        i += 1;
        if i > 30 {
            break;
        }
    }

    Ok(())
}
