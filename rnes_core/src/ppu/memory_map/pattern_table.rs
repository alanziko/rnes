#![allow(unused_parens)]

use bitflags::bitflags;

struct CHR {
    plane0: [u8; 8],
    plane1: [u8; 8],
}

impl CHR {
    pub fn new() -> Self {
        return Self { plane0: [0u8; 8], plane1: [0u8; 8] }
    }

    pub fn get_color_index(&self, address: CHRCoord) -> u8 {
        let p0 = self.plane0[address.y() as usize] & address.xth_bit();
        let p1 = self.plane1[address.y() as usize] & address.xth_bit();

        let index = (p1 << 1 | p0);
        return index;
    }
}

enum CoordError {
    OutOfBounds { x: u8, y: u8 },
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct CHRCoord: u8 {
        const X_MASK = 0b00001111;
        const Y_MASK = 0b11110000;
    }
}

impl CHRCoord {
    pub fn new(x: u8, y: u8) -> Result<Self, CoordError> {
        if(x > 7 || y > 7) {
            return Err(CoordError::OutOfBounds { x, y });
        }

        let bits = (x & 0x0F) | ((y << 4) & 0x0F);
        return Ok(Self::from_bits_retain(bits));        
    }

    pub fn from_u8(address: u8) -> Self {
        return Self::from_bits_retain(address);
    }

    pub fn x(&self) -> u8 {
        return (*self & Self::X_MASK).bits()
    }

    pub fn y(&self) -> u8 {
        return (*self & Self::X_MASK).bits() >> 4;
    }

    pub fn xth_bit(&self) -> u8 {
        return 0b1 >> self.x()
    }
}

pub struct PatternTable {
    tiles: [[CHR; 16]; 16]
}