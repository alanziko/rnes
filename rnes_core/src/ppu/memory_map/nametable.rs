use bitflags::bitflags;

use crate::ppu::memory_map::pattern_table::PatternTable;

struct AttributeTable {
    palette_data: [[AttrBlock; 8]; 8]
}

impl AttributeTable {
    pub fn new() -> Self {
        return Self { palette_data: [[AttrBlock::from_u8(0b00000000); 8]; 8] }
    }

    pub fn get_palette_at(&self, x: u8, y: u8) -> Result<u8, AttrBlockError> {
        let palette = &self.palette_data[(y >> 3) as usize][(x >> 3) as usize];

        return palette.at(x, y);
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct AttrBlock: u8 {
        const BOTTOMRIGHT_MASK =    0b11000000;
        const BOTTOMLEFT_MASK =     0b00110000;
        const TOPRIGHT_MASK =       0b00001100;
        const TOPLEFT_MASK =        0b00000011;
    }
}

enum AttrBlockError{
    AreaOutOfBounds { tl: u8, tr: u8, bl: u8, br: u8 },
    CoordOutOfBound { x: u8, y: u8 }
}

impl AttrBlock {
    pub fn new(top_left: u8, top_right: u8, bottom_left: u8, bottom_right: u8) -> Result<Self, AttrBlockError> {
        if(top_left > 3 || top_right > 3 || bottom_left > 3 || bottom_right > 3) {
            return Err(AttrBlockError::AreaOutOfBounds { 
                tl: top_left, 
                tr: top_right, 
                bl: bottom_left, 
                br: bottom_right 
            });
        }

        let area = (bottom_right << 6) | (bottom_left << 4) | (top_right << 2) | (top_left << 0);

        return Ok(AttrBlock::from_bits_retain(area));
    }

    pub fn from_u8(area: u8) -> Self {
        return AttrBlock::from_bits_retain(area);
    }

    pub fn top_left(&self) -> u8 {
        return (*self & Self::TOPLEFT_MASK).bits();
    }

    pub fn top_right(&self) -> u8 {
        return (*self & Self::TOPRIGHT_MASK).bits() >> 2;
    }

    pub fn bottom_left(&self) -> u8 {
        return (*self & Self::BOTTOMLEFT_MASK).bits() >> 4;
    }

    pub fn bottom_right(&self) -> u8 {
        return (*self & Self::BOTTOMRIGHT_MASK).bits() >> 6;
    }

    pub fn as_u8(&self) -> u8 {
        return (*self).bits();
    }

    pub fn at(&self, x: u8, y: u8) -> Result<u8, AttrBlockError> {
        if(x > 32 || y > 32) {
            return Err(AttrBlockError::CoordOutOfBound { x, y })
        }

        let closest_x = x % 4;
        let closest_y = y % 4;

        return Ok(match (closest_x >> 1, closest_y >> 1) {
            (0, 0) => self.top_left(),
            (1, 0) => self.top_right(),
            (0, 1) => self.bottom_left(),
            (1, 1) => self.top_right(),
            _ => unreachable!()
        })
    }
}

pub struct Nametable {
    tile_data: [[u8; 32]; 30],
    attr_table: AttributeTable
}

impl Nametable {
    pub fn new() -> Self {
        return Self {
            tile_data: [[0u8; 32]; 30],
            attr_table: AttributeTable::new()
        }
    }
}