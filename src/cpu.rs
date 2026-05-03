use bitflags::bitflags;

// TODO
// implement correct default values

#[derive(Default, Clone)]
pub struct CPU {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: StatusRegister,
}

bitflags! {
    #[derive(Clone)]
    pub struct StatusRegister: u8 {
        const Negative  = 0b10000000;
        const Overflow  = 0b01000000;
        const Ignored   = 0b00100000;
        const Break     = 0b00010000;
        const Decimal   = 0b00001000;
        const Interrupt = 0b00000100;
        const Zero      = 0b00000010;
        const Carry     = 0b00000001;
    }
}

impl Default for StatusRegister {
    fn default() -> Self {
        StatusRegister::empty()
    }
}
