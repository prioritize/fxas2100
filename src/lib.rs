#![no_std]
pub mod odr;
pub mod registers;
use crate::registers::*;
use embedded_hal_async::i2c::I2c;

pub struct Fxas2100<I> {
    i2c: I,
}

impl Fxas2100<I2c> {
    pub fn status() {}
    pub fn set_output_data_rate() {}
    pub fn read_byte() {}
    pub fn read_bytes() {}
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
