use tokio_linux_video::{types::*, Device};

#[cfg_attr(not(feature = "test-vivid"), ignore)]
#[tokio::test]
async fn output_hdmi() {
    let dev = env!("VIVID_HDMI_OUT");
    let dev = Device::open(dev).await.unwrap();

    let caps = dev.capabilities().await.unwrap();

    if !BufferType::VideoOutput.is_supported(caps.device_capabilities()) {
        panic!("Video output not supported");
    }

    let mut fmts = dev.formats(BufferType::VideoOutput);

    while let Some(fmt) = fmts.fetch_next().await.unwrap() {
        eprintln!("  fmt: {fmt}");
    }

    let mut prev_fmt = dev.format(BufferType::VideoOutput).await.unwrap();
    println!("fmt: {prev_fmt}");

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut fmt = Format::from(BufferType::VideoOutput);
    let pixfmt = fmt.try_mut::<PixFormat>().unwrap();
    pixfmt.set_color_space(ColorSpace::Srgb);
    pixfmt.set_pixel_format(FourCc::Rgb24);
    pixfmt.set_width(WIDTH);
    pixfmt.set_height(HEIGHT);
    //pixfmt.set_bytes_per_line(3 * pixfmt.width());
    //pixfmt.set_size_image(3 * pixfmt.width() * pixfmt.height());
    println!("new pixfmt: {pixfmt}");

    dev.try_format(&mut fmt).await.unwrap();
    dev.set_format(&mut fmt).await.unwrap();

    let fmt = dev.format(BufferType::VideoOutput).await.unwrap();
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
        let stream = dev.stream::<Out, Mmap>(ContentType::Video, 4).unwrap();

        let mut i = 0;
        loop {
            let buffer = stream.next().await.unwrap();
            let mut buffer = buffer.lock();
            println!("#{i} {buffer}");

            buffer.set_len(imgsize as _);

            let data: &mut [u8] = buffer.as_mut();
            println!("data len: {}", data.len());

            i += 1;
            if i > 5 {
                break;
            }
        }
    }

    dev.set_format(&mut prev_fmt).await.unwrap();
    //assert!(false);
}
