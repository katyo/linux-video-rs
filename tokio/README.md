# Tokio Linux V4L2 API for Rust

[![github](https://img.shields.io/badge/github-katyo/linux--video--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/linux-video-rs)
[![crate](https://img.shields.io/crates/v/tokio-linux-video.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/tokio-linux-video)
[![docs](https://img.shields.io/badge/docs.rs-tokio--linux--video-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/tokio-linux-video)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/actions/workflow/status/katyo/linux-video-rs/ci.yml?branch=master&style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/linux-video-rs/actions?query=workflow%3ARust)

This crates intended to provide access to Linux V4L2 APIs without any limitations.

The primary design goal is an optimal balance between safety and overhead.
The implementation much closer to system calls than v4l.
Interface types wraps kernel types to avoid unnecessary copying.

The secondary goal is providing full set of features of the original API.

At end this is my demure attempt to do things right.

## Crates

- [linux-video-core](https://crates.io/crates/linux-video-core) - core abstractions and low level interface (not for end users)
- [linux-video](https://crates.io/crates/linux-video) - sync interface which supports synchronous operation only
- **[tokio-linux-video](https://crates.io/crates/tokio-linux-video)** - async interface for [tokio](https://tokio.rs/) users
- [async-std-linux-video](https://crates.io/crates/async-std-linux-video) - async interface for [async-std](https://async.rs/) users

## Usage examples

Enumerating devices:

```rust,no_run
use tokio_linux_video::Device;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut devs = Device::list().await?;

    while let Some(path) = devs.fetch_next().await? {
        let dev = Device::open(&path).await?;

        let caps = dev.capabilities().await?;

        println!("path: {}, {caps}", path.display());
    }

    Ok(())
}
```

Getting capabilities and controls:

```rust,no_run
use tokio_linux_video::Device;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0").await?;

    let caps = dev.capabilities().await?;

    println!("Capabilities: {caps}");

    println!("Controls:");
    let mut controls = dev.controls(None);

    while let Some(ctrl) = controls.fetch_next().await? {
        println!("  {ctrl}");

        if let Some(mut items) = dev.control_items(&ctrl) {
            while let Some(item) = items.fetch_next().await? {
                println!("    {item}");
            }
        }
    }

    Ok(())
}
```

Getting supported formats:

```rust,no_run
use tokio_linux_video::{types::BufferType, Device};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0").await?;

    let caps = dev.capabilities().await?;

    for type_ in BufferType::ALL {
        if type_.is_supported(caps.capabilities()) {
            println!("{type_} formats:");
            let mut fmts = dev.formats(type_);

            if let Some(fmt) = fmts.fetch_next().await? {
                println!("  {fmt}");

                if type_.content().is_video() {
                    let mut sizes = dev.sizes(fmt.pixel_format());

                    while let Some(size) = sizes.fetch_next().await? {
                        println!("    {size}");

                        for size in size.sizes() {
                            println!("      {size}");
                            let mut intervals = dev.intervals(fmt.pixel_format(), size.width(), size.height());

                            while let Some(interval) = intervals.fetch_next().await? {
                                println!("        {interval}");
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
```

Using controls:

```rust,no_run
use tokio_linux_video::{types::*, Device};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0").await?;

    // Get control from device by identifier
    let contrast_ctrl = dev.control(CtrlId::Contrast).await?;

    // Create a value for control
    let mut contrast = Value::from(&contrast_ctrl);

    // Get control value from device
    dev.get_control(&mut contrast).await?;

    // Get reference to value data
    let contrast_value = contrast.try_ref::<i32>().unwrap();

    println!("Current contrast: {contrast_value:?}");

    // Set new value by reference
    *contrast.try_mut::<i32>().unwrap() = contrast_value + 10;

    println!("Updated contrast: {:?}", contrast.try_ref::<i32>().unwrap());

    // Set new control value to device
    dev.set_control(&contrast).await?;

    Ok(())
}
```

Capture video data:

```rust,no_run
use tokio_linux_video::{types::*, Device};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0").await?;

    // Get current format
    let mut fmt = dev.format(BufferType::VideoCapture).await?;
    println!("  {fmt}");

    // Start video capture stream
    let stream = dev.stream::<In, Mmap>(ContentType::Video, 4)?;

    let mut i = 0;
    while let Ok(buffer) = stream.next().await {
        let buffer = buffer.lock();
        println!("#{i} {buffer}");

        // Get reference to frame buffer contents
        let _data: &[u8] = buffer.as_ref();

        i += 1;
        if i > 30 {
            break;
        }
    }

    Ok(())
}
```


Output video data:

```rust,no_run
use tokio_linux_video::{types::*, Device};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0").await?;

    // Get current format
    let mut fmt = dev.format(BufferType::VideoOutput).await?;
    println!("  {fmt}");

    // Start video output stream
    let stream = dev.stream::<Out, Mmap>(ContentType::Video, 4)?;

    let mut i = 0;
    while let Ok(mut buffer) = stream.next().await {
        let mut buffer = buffer.lock();
        println!("#{i} {buffer}");

        // Get reference to frame buffer contents
        let _data: &mut [u8] = buffer.as_mut();

        i += 1;
        if i > 30 {
            break;
        }
    }

    Ok(())
}
```
