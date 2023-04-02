use linux_video::{types::*, Device};

#[cfg_attr(not(feature = "test-vivid"), ignore)]
#[test]
fn capture_webcam() {
    let dev = env!("VIVID_WEBCAM");
    let dev = Device::open(dev).unwrap();

    let caps = dev.capabilities().unwrap();

    if !BufferType::VideoCapture.is_supported(caps.device_capabilities()) {
        panic!("Video capture not supported");
    }

    let mut fmts = dev.formats(BufferType::VideoCapture);

    while let Some(fmt) = fmts.fetch_next().unwrap() {
        eprintln!("  fmt: {fmt}");
    }

    let mut prev_fmt = dev.format(BufferType::VideoCapture).unwrap();
    println!("fmt: {prev_fmt}");

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut fmt = Format::from(BufferType::VideoCapture);
    let pixfmt = fmt.try_mut::<PixFormat>().unwrap();
    pixfmt.set_color_space(ColorSpace::Srgb);
    pixfmt.set_pixel_format(FourCc::Rgb24);
    pixfmt.set_width(WIDTH);
    pixfmt.set_height(HEIGHT);
    //pixfmt.set_bytes_per_line(3 * pixfmt.width());
    //pixfmt.set_size_image(3 * pixfmt.width() * pixfmt.height());
    println!("new pixfmt: {pixfmt}");

    dev.try_format(&mut fmt).unwrap();
    dev.set_format(&mut fmt).unwrap();

    let fmt = dev.format(BufferType::VideoCapture).unwrap();
    println!("new fmt: {fmt}");

    let pixfmt = fmt.try_ref::<PixFormat>().unwrap();
    let width = pixfmt.width();
    let height = pixfmt.height();
    let rowsize = pixfmt.bytes_per_line();
    let imgsize = pixfmt.size_image();

    println!("{width}x{height} #{rowsize}/{imgsize}");

    assert_eq!(width, WIDTH);
    assert_eq!(height, HEIGHT);
    assert_eq!(rowsize, 3 * WIDTH);
    assert_eq!(imgsize, 3 * WIDTH * HEIGHT);

    {
        let stream = dev.stream::<In, Mmap>(ContentType::Video, 4).unwrap();

        let mut i = 0;
        loop {
            let buffer = stream.next().unwrap();
            let buffer = buffer.lock();
            println!("#{i} {buffer}");

            let data: &[u8] = buffer.as_ref();
            let _imgbuf =
                image::ImageBuffer::<image::Rgb<u8>, _>::from_raw(width, height, data).unwrap();
            //_imgbuf.save(format!("frame_{i:04}.png")).unwrap();

            i += 1;
            if i > 5 {
                break;
            }
        }
    }

    dev.set_format(&mut prev_fmt).unwrap();
    //assert!(false);
}
