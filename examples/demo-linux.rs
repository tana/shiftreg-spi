use std::{time::Duration, thread};

use embedded_hal::digital::OutputPin;
use linux_embedded_hal::SpidevDevice;
use shiftreg_spi::SipoShiftReg;

fn main() {
    let spi = SpidevDevice::open("/dev/spidev0.0").unwrap();

    // Initialize a 8-bit shift register
    let shift_reg: SipoShiftReg<_, 8, 1> = SipoShiftReg::new(spi);
    let mut pins = shift_reg.split();

    loop {
        for i in 0..8 {
            pins[i].set_high().unwrap();
            thread::sleep(Duration::from_millis(500));
            pins[i].set_low().unwrap();
        }
    }
}