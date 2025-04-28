// pub const STATUS: u8 = 0x00;
// pub const OUT_X_MSB: u8 = 0x01;
// pub const OUT_X_LSB: u8 = 0x02;
// pub const OUT_Y_MSB: u8 = 0x03;
// pub const OUT_Y_LSB: u8 = 0x04;
// pub const OUT_Z_MSB: u8 = 0x05;
// pub const OUT_Z_LSB: u8 = 0x06;
// pub const DR_STATUS: u8 = 0x07;
// pub const F_STATUS: u8 = 0x08;
// pub const F_SETUP: u8 = 0x09;
// pub const F_EVENT: u8 = 0x0A;
// pub const INT_SRC_FLAG: u8 = 0x0B;
// pub const WHO_AM_I: u8 = 0x0C;
// pub const CTRL_REG0: u8 = 0x0D;
// pub const RT_CFG: u8 = 0x0E;
// pub const RT_SRC: u8 = 0x0F;
// pub const RT_THS: u8 = 0x10;
// pub const RT_COUNT: u8 = 0x11;
// pub const TEMP: u8 = 0x12;
// pub const CTRL_REG1: u8 = 0x13;
// pub const CTRL_REG2: u8 = 0x14;
// pub const CTRL_REG3: u8 = 0x15;
// pub const ODR_RATE_800: u8 = 0;
// pub const ODR_RATE_400: u8 = 1;
// pub const ODR_RATE_200: u8 = 2;
// pub const ODR_RATE_100: u8 = 3;
// pub const ODR_RATE_50: u8 = 4;
// pub const ODR_RATE_25: u8 = 5;
// pub const ODR_RATE_12HALF: u8 = 6;
// pub const ODR_RATE_12HALF_1: u8 = 7;

pub enum Registers {
    STATUS,
    OutXMsb,
    OutXLsb,
    OutYMsb,
    OutYLsb,
    OutZMsb,
    OutZLsb,
    DrStatus,
    FStatus,
    FSetup,
    FEvent,
    IntSrcFlag,
    WhoAmI,
    CtrlReg0,
    RtCfg,
    RtSrc,
    RtThs,
    RtCount,
    Temp,
    CtrlReg1,
    CtrlReg2,
    CtrlReg3,
}
impl Registers {
    pub const fn to_u8(&self) -> u8 {
        match self {
            Registers::STATUS => 0x00,
            Registers::OutXMsb => 0x01,
            Registers::OutXLsb => 0x02,
            Registers::OutYMsb => 0x03,
            Registers::OutYLsb => 0x04,
            Registers::OutZMsb => 0x05,
            Registers::OutZLsb => 0x06,
            Registers::DrStatus => 0x07,
            Registers::FStatus => 0x08,
            Registers::FSetup => 0x09,
            Registers::FEvent => 0x0A,
            Registers::IntSrcFlag => 0x0B,
            Registers::WhoAmI => 0x0C,
            Registers::CtrlReg0 => 0x0D,
            Registers::RtCfg => 0x0E,
            Registers::RtSrc => 0x0F,
            Registers::RtThs => 0x10,
            Registers::RtCount => 0x11,
            Registers::Temp => 0x12,
            Registers::CtrlReg1 => 0x13,
            Registers::CtrlReg2 => 0x14,
            Registers::CtrlReg3 => 0x15,
        }
    }
}
