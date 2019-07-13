extern crate libusb;
extern crate log;

use std::time::Duration;

const VID: u16 = 0x1e71;
const PID: u16 = 0x170e;

pub struct Kraken<'a> {
    device: libusb::Device<'a>,
    iface: u8,
}

impl<'a> Kraken<'a> {
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

    pub fn set_fan(&self, speed: u8) -> libusb::Result<()> {
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
            Err(libusb::Error::Success) | Ok(()) => log::info!("Interface released successfully"),
            Err(e) => log::warn!("Unable to release interface: {}", e),
        }

        Ok(())
    }

    pub fn set_pump(&self, speed: u8) -> libusb::Result<()> {
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
            Err(libusb::Error::Success) | Ok(()) => log::info!("Interface released successfully"),
            Err(e) => log::warn!("Unable to release interface: {}", e),
        }

        Ok(())
    }
}
