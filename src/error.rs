use core::fmt::Debug;

#[derive(Debug)]
pub enum ShiftRegError<SpiError> {
    Spi(SpiError),
}

impl<SpiError> embedded_hal::digital::Error for ShiftRegError<SpiError>
where
    SpiError: Debug,
{
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        embedded_hal::digital::ErrorKind::Other
    }
}
