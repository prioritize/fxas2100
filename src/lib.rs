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

#[derive(PartialEq)]
pub enum State {
    Active,
    Ready,
    Standby,
    Error,
}

pub enum Error<I2cError> {
    I2C(I2cError),
}

pub struct FXAS2100<I2C> {
    pub i2c: I2C,
    pub address: u8,
    pub state: State,
}

// This function accepts the bitmask in binary "e.g. - 0b00001111" and the existing value, and will
// toggle the bits in the mask off in the value and return that value
const fn toggle_off(mask: u8, value: u8) -> u8 {
    !mask & value
}
// This function accepts the bitmask in binary "e.g. - 0b00001111" and the existing value, and will
// toggle the bits in the mask on in the value and return that value
const fn toggle_on(mask: u8, value: u8) -> u8 {
    mask | value
}

impl<I2C, E> FXAS2100<I2C>
where
    I2C: embedded_hal_async::i2c::I2c<Error = E>,
{
    pub async fn new(i2c: I2C, address: u8) -> Self {
        let mut fxas = Self {
            i2c,
            address,
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
    pub async fn set_active(&mut self) {
        let current_state = self.read_register(CTRL_REG1.to_u8()).await;
        let _ = self
            .set_register(CTRL_REG1.to_u8(), toggle_on(0x02, current_state))
            .await;
        self.state = State::Active;
    }
    pub async fn set_standby(&mut self) {
        let current_state = self.read_register(CTRL_REG1.to_u8()).await;
        let _ = self
            .set_register(CTRL_REG1.to_u8(), toggle_off(0x03, current_state))
            .await;
        self.state = State::Standby;
    }
    pub async fn set_ready(&mut self) {
        let current_state = self.read_register(CTRL_REG1.to_u8()).await;
        let _ = self
            .set_register(CTRL_REG1.to_u8(), toggle_on(0x01, current_state))
            .await;
        self.state = State::Ready;
    }
    pub async fn collect_gyro_data(&mut self) -> [u8; 6] {
        let mut data = [0u8; 6];
        self.read_bytes(OUT_X_MSB.to_u8(), &mut data).await;
        data
    }
    pub async fn enable_self_test(&mut self) {
        let mut reg_state = [0u8; 1];
        self.read_bytes(CTRL_REG1.to_u8(), &mut reg_state).await;
        self.set_register(CTRL_REG1.to_u8(), toggle_on(0x20, reg_state[0]))
            .await;
    }
}
impl<I2C, E> FXAS2100<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    pub fn new_blocking(i2c: I2C, address: u8, state: State) -> Self {
        Self {
            i2c,
            address,
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
