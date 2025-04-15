#![no_std]
pub mod odr;
pub mod registers;
use crate::registers::*;
use defmt::println;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::signal::Signal;
use odr::DataRate;

pub enum Error<I2cError> {
    I2C(I2cError),
}

pub struct FXAS2100<I2C> {
    pub i2c: I2C,
    pub address: u8,
    pub collect_signal: &'static Signal<CriticalSectionRawMutex, bool>,
}

// impl<I2C, E> embedded_hal_async::i2c::ErrorType for FXAS2100<I2C>
// where
//     I2C: embedded_hal_async::i2c::I2c<Error = E>,
// {
//     type Error = core::convert::Infallible;
// }
// impl I2c for FXAS2100<I2C>
// where
//     I2C: I2c,
// {
//     async fn transaction(
//         &mut self,
//         address: u8,
//         operations: &mut [embedded_hal_async::i2c::Operation<'_>],
//     ) -> Result<(), Self::Error> {
//         todo!()
//     }
// }
impl<I2C, E> FXAS2100<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub fn new(
        i2c: I2C,
        address: u8,
        collect_signal: &'static Signal<CriticalSectionRawMutex, bool>,
    ) -> Self {
        Self {
            i2c,
            address,
            collect_signal,
        }
    }
    pub fn status() {}
    pub async fn who_am_i(&mut self) -> u8 {
        self.read_register(registers::WHO_AM_I).await
    }
    pub async fn set_register(&mut self, register: u8, value: u8) {
        let _ = self.i2c.write(self.address, &[register, value]).await;
    }
    pub fn set_output_data_rate() {}
    pub fn read_byte() {}
    pub async fn set_odr(&mut self, rate: DataRate) {
        let mut register_state = 0u8;
        println!("odr_register: {}", register_state);
        let _ = self
            .read_bytes(registers::CTRL_REG1, &mut [register_state])
            .await;
        register_state |= odr::ODR_MASK;
        println!("odr_register: {}", register_state);
    }
    pub async fn read_bytes(&mut self, register: u8, buffer: &mut [u8]) {
        let _ = self.i2c.write_read(self.address, &[register], buffer).await;
    }

    pub async fn read_register(&mut self, register: u8) -> u8 {
        let mut data = [0u8; 1];
        match self
            .i2c
            .write_read(self.address, &[register], &mut data)
            .await
        {
            Ok(b) => println!("{}", b),
            Err(e) => println!("error"),
        }

        defmt::println!("{}", data);
        data[0]
    }
    pub fn read_temp() {}
    async fn set_active(&mut self) {
        let current_state = self.read_register(crate::registers::CTRL_REG1).await;
        let _ = self
            .i2c
            .write(self.address, &[
                registers::CTRL_REG1,
                current_state | 0b00000010,
            ])
            .await;
    }
    pub async fn signal_active(&self) {
        self.collect_signal.signal(true);
    }
    pub async fn signal_inactive(&self) {
        self.collect_signal.signal(false);
    }

    async fn set_inactive(&mut self) {
        let current_state = self.read_register(crate::registers::CTRL_REG1).await;
        let _ = self.i2c.write(self.address, &[
            registers::CTRL_REG1,
            current_state & 0b11111101,
        ]);
        self.collect_signal.signal(false);
    }
    pub async fn collect_gyro_data(&mut self, collect_signal: Signal<NoopRawMutex, bool>) {
        if collect_signal.signaled() {
            let mut gyro_data = [0u8; 6];
            let _ = self
                .i2c
                .write_read(self.address, &[registers::OUT_X_MSB], &mut gyro_data)
                .await;
        }
    }
    pub async fn state_handler(&mut self) {
        loop {
            self.collect_signal.wait().await;
            match self.collect_signal.signaled() {
                true => match self.collect_signal.try_take() {
                    Some(v) => match v {
                        true => self.set_active().await,
                        false => self.set_inactive().await,
                    },
                    None => println!("Unsuccessful try_take() in fxas::state_handler"),
                },
                false => println!("Doing a bunch of stuff for no reason"),
            }
        }
    }
}
impl<I2C, E> FXAS2100<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub fn new_blocking(
        i2c: I2C,
        address: u8,
        collect_signal: &'static mut Signal<CriticalSectionRawMutex, bool>,
    ) -> Self {
        Self {
            i2c,
            address,
            collect_signal,
        }
    }
    pub fn blocking_read_register(&mut self, register: u8) -> u8 {
        let mut data = 0x0;
        let _ = self.i2c.write_read(self.address, &[register], &mut [data]);
        data
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
