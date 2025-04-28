pub enum Mode {
    Circular,
    Stop(u8),
    Disabled,
}

impl Mode {
    pub const fn to_u8(&self) -> u8 {
        match self {
            Mode::Circular => 0b01,
            Mode::Stop(_) => 0b10,
            Mode::Disabled => 0b00,
        }
    }
}
