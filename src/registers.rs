pub const STATUS: u8 = 0x00;
pub const OUT_X_MSB: u8 = 0x01;
pub const OUT_X_LSB: u8 = 0x02;
pub const OUT_Y_MSB: u8 = 0x03;
pub const OUT_Y_LSB: u8 = 0x04;
pub const OUT_Z_MSB: u8 = 0x05;
pub const OUT_Z_LSB: u8 = 0x06;
pub const DR_STATUS: u8 = 0x07;
pub const F_STATUS: u8 = 0x08;
pub const F_SETUP: u8 = 0x09;
pub const F_EVENT: u8 = 0x0A;
pub const INT_SRC_FLAG: u8 = 0x0B;
pub const WHO_AM_I: u8 = 0x0C;
pub const CTRL_REG0: u8 = 0x0D;
pub const RT_CFG: u8 = 0x0E;
pub const RT_SRC: u8 = 0x0F;
pub const RT_THS: u8 = 0x10;
pub const RT_COUNT: u8 = 0x11;
pub const TEMP: u8 = 0x12;
pub const CTRL_REG1: u8 = 0x13;
pub const CTRL_REG2: u8 = 0x14;
pub const CTRL_REG3: u8 = 0x15;

pub enum Registers {
    STATUS,
    OUT_X_MSB,
    OUT_X_LSB,
    OUT_Y_MSB,
    OUT_Y_LSB,
    OUT_Z_MSB,
    OUT_Z_LSB,
    DR_STATUS,
    F_STATUS,
    F_SETUP,
    F_EVENT,
    INT_SRC_FLAG,
    WHO_AM_I,
    CTRL_REG0,
    RT_CFG,
    RT_SRC,
    RT_THS,
    RT_COUNT,
    TEMP,
    CTRL_REG1,
    CTRL_REG2,
    CTRL_REG3,
}
