#![no_std]
pub mod odr;
pub mod registers;
use crate::registers::*;
use defmt::println;
use embedded_hal_async::i2c::I2c;

pub enum Error<I2cError> {
    I2C(I2cError),
}
pub struct FXAS2100<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C> FXAS2100<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { i2c, address }
    }
    pub fn status() {}
    pub fn set_output_data_rate() {}
    pub fn read_byte() {}
    pub async fn read_register(&mut self, register: u8) -> u8 {
        let data = 17u8;
        match self
            .i2c
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
