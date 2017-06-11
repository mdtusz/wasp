# Rusty Print
## A 3D printer driver firmware written in Rust

### Supported boards
 - Teensy 3.2

### How to build
Install:
 - Current nightly build of Rust
 - [Xargo](https://github.com/japaric/xargo)
 - The arm-none-eabi-gcc toolchain
 - Clang, see [rust-bindgen’s requirements](https://github.com/servo/rust-bindgen#requirements).
 - [teensy-loader-cli](https://www.pjrc.com/teensy/loader_cli.html) for flashing your program onto hardware.