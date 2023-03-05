use linux_video::{types::*, Device};

fn main() -> std::io::Result<()> {
    let dev = Device::open("/dev/video0")?;

    // Get control from device by identifier
    let contrast_ctrl = dev.control(CtrlId::Contrast)?;

    // Create a value for control
    let mut contrast = Value::from(&contrast_ctrl);

    // Get control value from device
    dev.get_control(&mut contrast)?;

    // Get reference to value data
    let contrast_value = contrast.try_ref::<i32>().unwrap();

    println!("Current contrast: {contrast_value:?}");

    // Set new value by reference
    *contrast.try_mut::<i32>().unwrap() = contrast_value + 10;

    println!("Updated contrast: {:?}", contrast.try_ref::<i32>().unwrap());

    // Set new control value to device
    dev.set_control(&contrast)?;

    Ok(())
}
