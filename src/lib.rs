#![no_std]
pub mod odr;
pub mod registers;
use crate::registers::*;
use defmt::println;
use embedded_hal_async::i2c::{Error as I2cError, I2c};

pub enum Error<I2cError> {
    I2C(I2cError),
}

pub struct FXAS2100 {
    address: u8,
}

impl embedded_hal_async::i2c::ErrorType for FXAS2100 {
    type Error = core::convert::Infallible;
}
impl I2c for FXAS2100 {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal_async::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
impl FXAS2100 {
    pub fn new(address: u8) -> Self {
        Self { address }
    }
    pub fn status() {}
    pub fn set_output_data_rate() {}
    pub fn read_byte() {}
    pub async fn read_register(&mut self, register: u8) -> u8 {
        let data = 17u8;
        match self
            .write_read(self.address, &[register], &mut [data])
            .await
        {
            Ok(b) => println!("{}", b),
            Err(e) => println!("error"),
        }

        defmt::println!("{}", data);
        data
    }
    pub fn read_temp() {}
    pub fn set_active() {}
    pub fn set_inactive() {}
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
