#![no_std]
pub mod commands;
pub mod odr;
pub mod outputs;
pub mod registers;
use crate::registers::Registers::*;
use defmt::println;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer};
use odr::DataRate;

pub enum State {
    Active,
    Ready,
    Standby,
}

pub enum Error<I2cError> {
    I2C(I2cError),
}

pub struct FXAS2100<I2C> {
    pub i2c: I2C,
    pub address: u8,
    pub collect_signal: &'static Signal<CriticalSectionRawMutex, bool>,
    pub state: State,
}

// impl<I2C, E> embedded_hal_async::i2c::ErrorType for FXAS2100<I2C>
// where
//     I2C: embedded_hal_async::i2c::I2c<Error = E>,
// {
//     type Error = core::convert::Infallible;
// }
// impl I2c for FXAS2100<I2C>where
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
    pub async fn new(
        i2c: I2C,
        address: u8,
        collect_signal: &'static Signal<CriticalSectionRawMutex, bool>,
    ) -> Self {
        let mut fxas = Self {
            i2c,
            address,
            collect_signal,
            state: State::Standby,
        };
        let mut ctrl_reg = [0u8; 1];
        fxas.set_register(CTRL_REG1.to_u8(), 0b01000000).await;
        Timer::after_millis(50).await;
        fxas.read_bytes(CTRL_REG1.to_u8(), &mut ctrl_reg).await;
        assert_eq!(ctrl_reg[0], 0);
        fxas
    }
    pub async fn status(&mut self) -> u8 {
        let mut data = [0u8; 1];
        self.read_bytes(STATUS.to_u8(), &mut data).await;
        data[0]
    }
    pub async fn who_am_i(&mut self) -> u8 {
        self.read_register(WHO_AM_I.to_u8()).await
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
            .read_bytes(CTRL_REG1.to_u8(), &mut [register_state])
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
        let current_state = self.read_register(CTRL_REG1.to_u8()).await;
        let _ = self
            .set_register(CTRL_REG1.to_u8(), current_state | 0b00000010)
            .await;
        //self.
    }
    pub async fn signal_active(&self) {
        self.collect_signal.signal(true);
    }
    pub async fn signal_inactive(&self) {
        self.collect_signal.signal(false);
    }

    async fn set_inactive(&mut self) {
        let current_state = self.read_register(CTRL_REG1.to_u8()).await;
        let _ = self
            .set_register(CTRL_REG1.to_u8(), current_state & 0xFC)
            .await;
        self.collect_signal.signal(false);
    }
    pub async fn collect_gyro_data(&mut self, collect_signal: Signal<NoopRawMutex, bool>) {
        if collect_signal.signaled() {
            let mut gyro_data = [0u8; 6];
            let _ = self.read_bytes(OUT_X_MSB.to_u8(), &mut gyro_data).await;
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
        state: State,
    ) -> Self {
        Self {
            i2c,
            address,
            collect_signal,
            state,
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
