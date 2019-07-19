//! # Example
//!
//! ```
//! use camctl_rs::Kraken;
//!
//! let ctx = libusb::Context::new().unwrap();
//! let kraken = Kraken::from(&ctx).unwrap();
//!
//! kraken.set_fan(75); // sets the fan speed to 75%
//! ```
mod cam;
pub use cam::*;
