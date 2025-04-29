pub enum DataRate {
    EightHundred,
    FourHundred,
    TwoHundred,
    OneHundred,
    Fifty,
    TwentyFive,
    TwelveAndAHalf,
}
pub const ODR_MASK: u8 = 0b00011100;
impl DataRate {
    pub const fn to_mask(&self) -> u8 {
        match self {
            DataRate::EightHundred => 0b000,
            DataRate::FourHundred => 0b001,
            DataRate::TwoHundred => 0b010,
            DataRate::OneHundred => 0b011,
            DataRate::Fifty => 0b100,
            DataRate::TwentyFive => 0b101,
            DataRate::TwelveAndAHalf => 0b110,
        }
    }
    pub const fn to_u16(&self) -> u16 {
        match self {
            DataRate::EightHundred => 800,
            DataRate::FourHundred => 400,
            DataRate::TwoHundred => 200,
            DataRate::OneHundred => 100,
            DataRate::Fifty => 50,
            DataRate::TwentyFive => 25,
            DataRate::TwelveAndAHalf => 12,
        }
    }
}
