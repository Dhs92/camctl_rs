# camctl_rs
[![crates.io](https://img.shields.io/crates/v/camctl_rs.svg)](https://crates.io/crates/camctl_rs)
[![License](https://img.shields.io/github/license/Dhs92/camctl_rs.svg)](LICENSE)
[![Issues](https://img.shields.io/github/issues/Dhs92/camctl_rs.svg)](https://github.com/Dhs92/camctl_rs/issues)

A Rust rewrite of https://github.com/leaty/camctl
A small library that gives developers a way to interface with Kraken X62 AIO CPU coolers

# Usage
```rust
    use camctl_rs::cam::Kraken;

    let ctx = libusb::Context::new().unwrap();
    let kraken = Kraken::from(&ctx).unwrap();

    kraken.set_fan(75); // sets the fan speed to 75%
```

# TODO
- Timeout configurable
- More logging

