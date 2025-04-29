pub enum Mode {
    Circular,
    Stop(u8),
    Disabled,
}

impl Mode {
    pub const fn to_mask(&self) -> u8 {
        match self {
            Mode::Circular => 64,
            Mode::Stop(_) => 128,
            Mode::Disabled => 00,
        }
    }
}
