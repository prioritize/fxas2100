pub enum Masks {
    Reset,
    SelfTest,
    OutputDataRate,
    Active,
    Ready,
    FifoMode,
    FifoWatermark,
    FifoEvent,
    FifoEventTime,
    FifoOverflow,
    FifoWatermarkEnable,
    FifoCount,
}
impl Masks {
    pub const fn to_mask(&self) -> u8 {
        match self {
            Masks::Reset => 64,
            Masks::SelfTest => 32,
            Masks::OutputDataRate => 28,
            Masks::Active => 2,
            Masks::Ready => 1,
            Masks::FifoMode => 192,
            Masks::FifoWatermark => 63,
            Masks::FifoEvent => 32,
            Masks::FifoEventTime => 31,
            Masks::FifoOverflow => 128,
            Masks::FifoWatermarkEnable => 64,
            Masks::FifoCount => 63,
        }
    }
}
