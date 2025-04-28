#![no_std]
pub mod commands;
pub mod fifo;
pub mod masks;
pub mod odr;
pub mod outputs;
pub mod registers;
use crate::registers::Registers::*;
use defmt::println;
use embassy_time::Timer;
use fifo::Mode;
use masks::Masks;
use odr::{DataRate, ODR_MASK};

pub const DEFAULT_ADDRESS: u8 = 0x21;

#[derive(Debug, Clone)]
pub enum FXASError {
    ResetError,
    FifoConfigureError,
}
type Result<T> = core::result::Result<T, FXASError>;
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
    pub data_rate: DataRate,
}

// This function accepts the bitmask in binary "e.g. - 0b00001111" and the existing value, and will
// toggle the bits in the mask off in the value and return that value
#[inline]
const fn toggle_off(mask: u8, value: u8) -> u8 {
    !mask & value
}
// This function accepts the bitmask in binary "e.g. - 0b00001111" and the existing value, and will
// toggle the bits in the mask on in the value and return that value
#[inline]
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
            data_rate: DataRate::EightHundred,
        };
        let _ = fxas.reset().await;
        fxas
    }
    pub async fn reset(&mut self) -> Result<()> {
        let mut data = [0u8; 1];
        self.set_register(CtrlReg1.to_u8(), Masks::Reset.to_mask())
            .await;
        self.read_bytes(CtrlReg1.to_u8(), &mut data).await;
        Timer::after_millis(50).await;
        match 0x00 == data[0] {
            true => Ok(()),
            false => {
                println!("FXAS21002 failed to reset correctly");
                Err(FXASError::ResetError)
            }
        }
    }
    pub async fn configure_fifo_mode(&mut self, mode: Mode) -> Result<()> {
        let mut f_setup = [0u8; 1];
        self.set_standby().await;
        self.read_bytes(FSetup.to_u8(), &mut f_setup).await;
        self.set_register(FSetup.to_u8(), toggle_on(f_setup[0], mode.to_u8()))
            .await;
        self.read_bytes(FSetup.to_u8(), &mut f_setup).await;
        f_setup[0] &= mode.to_u8();

        if f_setup[0] != mode.to_u8() {
            Err(FXASError::FifoConfigureError)
        } else {
            Ok(())
        }
    }
    pub async fn status(&mut self) -> u8 {
        let mut data = [0u8; 1];
        self.read_bytes(STATUS.to_u8(), &mut data).await;
        data[0]
    }
    pub async fn who_am_i(&mut self) -> u8 {
        self.read_register(WhoAmI.to_u8()).await
    }
    pub async fn set_register(&mut self, register: u8, value: u8) {
        let _ = self.i2c.write(self.address, &[register, value]).await;
    }
    pub async fn set_odr(&mut self, rate: DataRate) {
        let ack_rate = rate.to_u8();
        let mut register_state = 0u8;
        println!("odr_register: {}", register_state);
        let _ = self
            .read_bytes(CtrlReg1.to_u8(), &mut [register_state])
            .await;
        println!("current odr setting: {}", register_state & ODR_MASK);
        register_state = toggle_on(toggle_off(!ODR_MASK, register_state), ack_rate);
        self.set_register(CtrlReg1.to_u8(), toggle_on(ODR_MASK, register_state))
            .await;
        self.data_rate = rate;
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
            Err(_) => println!("error"),
        }

        defmt::println!("{}", data);
        data[0]
    }
    pub async fn read_temp(&mut self) -> Result<u8> {
        let mut temp = [0u8; 1];
        self.read_bytes(Temp.to_u8(), &mut temp).await;
        Ok(temp[0])
    }
    pub async fn set_active(&mut self) {
        let current_state = self.read_register(CtrlReg1.to_u8()).await;
        let _ = self
            .set_register(CtrlReg1.to_u8(), toggle_on(0x02, current_state))
            .await;
        self.state = State::Active;
    }
    pub async fn set_standby(&mut self) {
        let current_state = self.read_register(CtrlReg1.to_u8()).await;
        let _ = self
            .set_register(CtrlReg1.to_u8(), toggle_off(0x03, current_state))
            .await;
        self.state = State::Standby;
    }
    pub async fn set_ready(&mut self) {
        let current_state = self.read_register(CtrlReg1.to_u8()).await;
        let _ = self
            .set_register(CtrlReg1.to_u8(), toggle_on(0x01, current_state))
            .await;
        self.state = State::Ready;
    }
    pub async fn collect_gyro_data(&mut self) -> [u8; 6] {
        let mut data = [0u8; 6];
        self.read_bytes(OutXMsb.to_u8(), &mut data).await;
        data
    }
    pub async fn get_fifo_count(&mut self) -> u8 {
        let mut sample_count = 0u8;
        self.read_bytes(FStatus.to_u8(), &mut [sample_count]).await;
        sample_count &= Masks::FifoCount.to_mask();
        sample_count
    }
    pub async fn enable_self_test(&mut self) {
        let mut reg_state = [0u8; 1];
        self.read_bytes(CtrlReg1.to_u8(), &mut reg_state).await;
        self.set_register(CtrlReg1.to_u8(), toggle_on(0x20, reg_state[0]))
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
            data_rate: DataRate::EightHundred,
        }
    }
    pub fn blocking_read_register(&mut self, register: u8) -> u8 {
        let data = 0x0;
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
