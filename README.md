# SPI-based driver for shift registers
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/tana/shiftreg-spi/rust.yml)
![docs.rs](https://img.shields.io/docsrs/shiftreg-spi)

This is an embedded Rust driver for shift registers (such as 74HC595).

## Features
- Fast communication using SPI peripheral.
- Implements [`OutputPin` trait](https://docs.rs/embedded-hal/1.0.0-rc.1/embedded_hal/digital/trait.OutputPin.html).
  - Pins of shift registers can be accessed (and passed to other libraries) like microcontroller pins.
- Supports unlimited daisy chain.
  - Size is specified using const generics

## Usage
```rust
// Initialize a 8-bit shift register
let shift_reg: SipoShiftReg<_, 8, 1> = SipoShiftReg::new(spi);
// Get an array of individual pins
let mut pins = shift_reg.split();

// Set state of a pin
pins[0].set_high().unwrap();
pins[1].set_low().unwrap();
```
`spi` is an SPI driver object which implements [`SpiDevice`](https://docs.rs/embedded-hal/1.0.0-rc.1/embedded_hal/spi/trait.SpiDevice.html).
