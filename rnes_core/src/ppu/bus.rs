use crate::{bus::Bus, ppu::memory_map::{nametable::Nametable, pattern_table::PatternTable}};

struct PPUBus {
    pattern_tables: [PatternTable; 2],
    nametables: [Nametable; 4]
}