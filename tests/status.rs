use flexi_logger::{opt_format, Logger};

#[test]
fn status() -> libusb::Result<()> {
    Logger::with_env_or_str("debug")
        .log_to_file()
        .directory("LOGS")
        .format(opt_format)
        .start()
        .unwrap();

    let ctx = libusb::Context::new().unwrap();
    let kraken = camctl_rs::Kraken::from(&ctx).unwrap();
    let kraken_info = kraken.status()?;

    println!("Liquid Temp: {}C", kraken_info.liquid_temp);
    println!("Fan Speed: {} RPM", kraken_info.fan_speed);
    println!("Pump Speed: {} RPM", kraken_info.pump_speed);

    Ok(())
}
