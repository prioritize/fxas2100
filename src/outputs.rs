use crate::registers;

pub enum GyroOutput {
    WhoAmI(u8),
    GyroData([u8; 6]),
    Temperature([u8; 1]),
    Register(registers::Registers, u8),
}
