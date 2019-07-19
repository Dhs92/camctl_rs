use std::time::Duration;

const VID: u16 = 0x1e71;
const PID: u16 = 0x170e;

pub struct Kraken<'a> {
    pub(crate) device: libusb::Device<'a>,
    pub(crate) iface: u8,
}

#[derive(Debug)]
pub struct Info {
    pub liquid_temp: f32,
    pub fan_speed: u16,
    pub pump_speed: u16,
}

impl Info {
    pub fn from(kraken: &Kraken) -> libusb::Result<Self> {
        let mut device_handle: libusb::DeviceHandle = kraken.device.open()?;
        let buf: &mut [u8; 17] = &mut [0; 17];
        let _ = device_handle.claim_interface(kraken.iface);
        let _bytes = match device_handle.read_bulk(0x81, buf, Duration::new(10, 0)) {
            Ok(u) => log::info!("Status buffer size: {} bytes", u),
            Err(e) => log::warn!("Could not read from endpoint: {}", e),
        };

        let result = Self {
            liquid_temp: (buf[1] as f32 + buf[2] as f32 / 10f32) as f32,
            fan_speed: (buf[3] as u16) << 8 | buf[4] as u16,
            pump_speed: (buf[5] as u16) << 8 | buf[6] as u16,
        };

        Ok(result)
    }
}
/// # Example
///
/// ```
/// use camctl_rs::cam::Kraken;
///
/// let ctx = libusb::Context::new().unwrap();
/// let kraken = Kraken::from(&ctx).unwrap();
///
/// kraken.set_fan(75); // sets the fan speed to 75%
/// ```
impl<'a> Kraken<'a> {
    /// Constructs a new `Kraken` from [libusb::Context](https://docs.rs/libusb/0.3.0/libusb/struct.Context.html)
    pub fn from(ctx: &'a libusb::Context) -> libusb::Result<Self> {
        let device = Kraken {
            iface: 0, // magic number
            device: Kraken::get_device(ctx, PID, VID)?,
        };
        Ok(device)
    }

    fn get_device(ctx: &'a libusb::Context, pid: u16, vid: u16) -> libusb::Result<libusb::Device> {
        //iterate through DeviceList provided by Context
        for device in ctx.devices()?.iter() {
            // grab information on current device
            let device_desc = device.device_descriptor()?;

            // check if device matches requested PID and VID
            if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
                return Ok(device);
            }
        }

        Err(libusb::Error::NotFound)
    }

    /// Set the fan speed by percent, `speed` should be 0-100
    ///
    /// If the speed is over 100, the method will return without side-effects
    pub fn set_fan(&self, speed: u8) -> libusb::Result<()> {
        match speed {
            0..=100 => {
                let mut interface = self.device.open()?;

                if interface.kernel_driver_active(self.iface)? {
                    interface.detach_kernel_driver(self.iface)?;
                }

                interface.claim_interface(self.iface)?;

                // magic payload, https://github.com/leaty/camctl/blob/master/camctl#L31
                let payload: [u8; 24] = [
                    2, 77, 0, 0, speed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];

                // write the payload, 10 second timeout
                match interface.write_bulk(1, &payload, Duration::new(10, 0)) {
                    Err(libusb::Error::Io) => {
                        log::warn!("Write failed!");
                        match interface.release_interface(self.iface) {
                            Err(libusb::Error::Success) | Ok(()) => {
                                log::info!("Interface released successfully")
                            }
                            Err(e) => log::warn!("Unable to release interface: {}", e),
                        }
                    }

                    Err(_) => {
                        log::error!("An unknown error has occurred");
                        match interface.release_interface(self.iface) {
                            Err(libusb::Error::Success) | Ok(()) => {
                                log::info!("Interface released successfully")
                            }
                            Err(e) => log::warn!("Unable to release interface: {}", e),
                        }
                    }
                    Ok(_) => (),
                };

                if !interface.kernel_driver_active(self.iface)? {
                    match interface.attach_kernel_driver(self.iface) {
                        Err(libusb::Error::Success) | Ok(()) => {
                            log::info!("Kernel driver re-attached successfully")
                        }
                        Err(e) => log::warn!("Kernel driver could not be re-attached: {}", e),
                    } // re-attach the kernel driver
                }

                match interface.release_interface(self.iface) {
                    Err(libusb::Error::Success) | Ok(()) => {
                        log::info!("Interface released successfully")
                    }
                    Err(e) => log::warn!("Unable to release interface: {}", e),
                }
            }
            _ => log::warn!("Speed provided was: {}", speed),
        }
        Ok(())
    }

    /// Set the pump speed by percent, `speed` should be 0-100
    ///
    /// If the speed is over 100, the method will return without side-effects
    pub fn set_pump(&self, speed: u8) -> libusb::Result<()> {
        match speed {
            0..=100 => {
                let mut interface = self.device.open()?;

                if interface.kernel_driver_active(self.iface)? {
                    interface.detach_kernel_driver(self.iface)?;
                }

                interface.claim_interface(self.iface)?;

                // magic payload, https://github.com/leaty/camctl/blob/master/camctl#L31
                let payload: [u8; 24] = [
                    2, 77, 64, 0, speed, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ];

                match interface.write_bulk(1, &payload, Duration::new(10, 0)) {
                    Err(libusb::Error::Io) => {
                        log::warn!("Write failed!");
                        match interface.release_interface(self.iface) {
                            Err(libusb::Error::Success) | Ok(()) => {
                                log::info!("Interface released successfully")
                            }
                            Err(e) => log::warn!("Unable to release interface: {}", e),
                        }
                    }

                    Err(_) => {
                        log::error!("An unknown error has occurred");
                        match interface.release_interface(self.iface) {
                            Err(libusb::Error::Success) | Ok(()) => {
                                log::info!("Interface released successfully")
                            }
                            Err(e) => log::warn!("Unable to release interface: {}", e),
                        }
                    }
                    Ok(_) => (),
                };

                if !interface.kernel_driver_active(self.iface)? {
                    match interface.attach_kernel_driver(self.iface) {
                        Err(libusb::Error::Success) | Ok(()) => {
                            log::info!("Kernel driver re-attached successfully")
                        }
                        Err(e) => log::warn!("Kernel driver could not be re-attached: {}", e),
                    } // re-attach the kernel driver
                }

                match interface.release_interface(self.iface) {
                    Err(libusb::Error::Success) | Ok(()) => {
                        log::info!("Interface released successfully")
                    }
                    Err(e) => log::warn!("Unable to release interface: {}", e),
                }
            }
            _ => log::warn!("Speed provided was: {}", speed),
        }

        Ok(())
    }

    pub fn status(&self) -> libusb::Result<Info> {
        Info::from(&self)
    }
}
