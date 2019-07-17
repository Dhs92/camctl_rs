#[test]
fn main() -> libusb::Result<()> {
    let ctx = libusb::Context::new().unwrap();
    let kraken = Kraken::from(&ctx).unwrap();
    let kraken_info = kraken.status()?;

    println!("Liquid Temp: {}C", kraken_info.liquid_temp);
    println!("Fan Speed: {} RPM", kraken_info.fan_speed);
    println!("Pump Speed: {} RPM", kraken_info.pump_speed);

    Ok(())
}