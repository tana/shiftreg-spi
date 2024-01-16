use core::cell::RefCell;

use critical_section::Mutex;
use embedded_hal::{
    digital::{ErrorType, OutputPin, PinState},
    spi::SpiDevice,
};

use crate::ShiftRegError;

// The design of this struct is inspired by `port-expander` clate ( https://crates.io/crates/port-expander ).
pub struct SipoShiftReg<Spi, const BITS: usize, const BYTES: usize>(
    Mutex<RefCell<SipoShiftRegInner<Spi, BITS, BYTES>>>,
);

impl<Spi, const BITS: usize, const BYTES: usize> SipoShiftReg<Spi, BITS, BYTES>
where
    Spi: SpiDevice,
{
    pub fn new(spi: Spi) -> Self {
        Self(Mutex::new(RefCell::new(SipoShiftRegInner {
            spi,
            state: [0; BYTES],
            lazy: false,
        })))
    }

    pub fn split<'a>(&'a self) -> [SipoShiftRegPin<'a, Spi, BITS, BYTES>; BITS] {
        core::array::from_fn(|i| SipoShiftRegPin {
            shift_reg: &self.0,
            idx: i,
        })
    }

    pub fn set_lazy(&self, lazy: bool) {
        critical_section::with(|cs| self.0.borrow_ref_mut(cs).lazy = lazy)
    }

    pub fn update(&self) -> Result<(), ShiftRegError<Spi::Error>> {
        critical_section::with(|cs| self.0.borrow_ref_mut(cs).update())
    }
}

struct SipoShiftRegInner<Spi, const BITS: usize, const BYTES: usize> {
    spi: Spi,
    state: [u8; BYTES],
    lazy: bool,
}

impl<Spi, const BITS: usize, const BYTES: usize> SipoShiftRegInner<Spi, BITS, BYTES>
where
    Spi: SpiDevice,
{
    fn set_pin_state(
        &mut self,
        idx: usize,
        state: PinState,
    ) -> Result<(), ShiftRegError<Spi::Error>> {
        let byte_idx = idx / 8;
        let bit_idx = idx % 8;

        match state {
            PinState::High => self.state[byte_idx] |= 1 << bit_idx,
            PinState::Low => self.state[byte_idx] &= !(1 << bit_idx),
        };

        if !self.lazy {
            self.update()?;
        }

        Ok(())
    }

    fn update(&mut self) -> Result<(), ShiftRegError<Spi::Error>> {
        self.spi.write(&self.state).map_err(ShiftRegError::Spi)
    }
}

pub struct SipoShiftRegPin<'a, Spi, const BITS: usize, const BYTES: usize> {
    shift_reg: &'a Mutex<RefCell<SipoShiftRegInner<Spi, BITS, BYTES>>>,
    idx: usize,
}

impl<'a, Spi, const BITS: usize, const BYTES: usize> ErrorType
    for SipoShiftRegPin<'a, Spi, BITS, BYTES>
where
    Spi: SpiDevice,
{
    type Error = ShiftRegError<Spi::Error>;
}

impl<'a, Spi, const BITS: usize, const BYTES: usize> OutputPin
    for SipoShiftRegPin<'a, Spi, BITS, BYTES>
where
    Spi: SpiDevice,
{
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::High)
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_state(PinState::Low)
    }

    fn set_state(&mut self, state: PinState) -> Result<(), Self::Error> {
        critical_section::with(|cs| {
            self.shift_reg
                .borrow_ref_mut(cs)
                .set_pin_state(self.idx, state)
        })
    }
}
