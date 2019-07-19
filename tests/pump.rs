use flexi_logger::{opt_format, Logger};

#[test]
fn pump() -> libusb::Result<()> {
    Logger::with_env_or_str("debug")
        .log_to_file()
        .directory("LOGS")
        .format(opt_format)
        .start()
        .unwrap();

    let ctx = libusb::Context::new().unwrap();
    let kraken = camctl_rs::Kraken::from(&ctx).unwrap();

    for i in 60..=100 {
        kraken.set_pump(i)?
    }

    Ok(())
}
