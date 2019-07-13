# camctl_rs
[![crates.io](https://img.shields.io/crates/v/camctl_rs.svg)](https://crates.io/crates/camctl_rs)
[![License](https://img.shields.io/github/license/Dhs92/camctl_rs.svg)](LICENSE.txt)
[![Issues](https://img.shields.io/github/issues/Dhs92/camctl_rs.svg)](https://github.com/Dhs92/camctl_rs/issues)

A Rust rewrite of https://github.com/leaty/camctl

# Usage
```rust
    use crate::camctl::Kraken;

    let ctx = libusb::Context::new()?;
    let kraken = Kraken::from(&ctx)?;

    kraken.set_fan(75); // sets the fan speed to 75%
```

# TODO
- Timeout configurable
- More logging
- Use self.interface instead of magic value

