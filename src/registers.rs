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
// pub const ODR_RATE_800: u8 = 0;
// pub const ODR_RATE_400: u8 = 1;
// pub const ODR_RATE_200: u8 = 2;
// pub const ODR_RATE_100: u8 = 3;
// pub const ODR_RATE_50: u8 = 4;
// pub const ODR_RATE_25: u8 = 5;
// pub const ODR_RATE_12HALF: u8 = 6;
// pub const ODR_RATE_12HALF_1: u8 = 7;

pub enum FXASRegisters {
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
impl FXASRegisters {
    pub const fn to_u8(&self) -> u8 {
        match self {
            FXASRegisters::STATUS => 0x00,
            FXASRegisters::OUT_X_MSB => 0x01,
            FXASRegisters::OUT_X_LSB => 0x02,
            FXASRegisters::OUT_Y_MSB => 0x03,
            FXASRegisters::OUT_Y_LSB => 0x04,
            FXASRegisters::OUT_Z_MSB => 0x05,
            FXASRegisters::OUT_Z_LSB => 0x06,
            FXASRegisters::DR_STATUS => 0x07,
            FXASRegisters::F_STATUS => 0x08,
            FXASRegisters::F_SETUP => 0x09,
            FXASRegisters::F_EVENT => 0x0A,
            FXASRegisters::INT_SRC_FLAG => 0x0B,
            FXASRegisters::WHO_AM_I => 0x0C,
            FXASRegisters::CTRL_REG0 => 0x0D,
            FXASRegisters::RT_CFG => 0x0E,
            FXASRegisters::RT_SRC => 0x0F,
            FXASRegisters::RT_THS => 0x10,
            FXASRegisters::RT_COUNT => 0x11,
            FXASRegisters::TEMP => 0x12,
            FXASRegisters::CTRL_REG1 => 0x13,
            FXASRegisters::CTRL_REG2 => 0x14,
            FXASRegisters::CTRL_REG3 => 0x15,
        }
    }
}
