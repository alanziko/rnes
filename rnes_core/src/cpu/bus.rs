use crate::bus::{Bus};

struct RAM {
    pub data: [u8; 2048],
}

impl RAM {
    pub fn new() -> Self {
        Self { data: [0u8; 2048] }
    }
}

pub struct CPUBus {
    ram: RAM,
}

impl CPUBus {
    pub fn new() -> Self {
        Self { ram: RAM::new() }
    }
}

impl Bus for CPUBus {
    fn get_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram.data[(address % 2048) as usize],
            _ => todo!(),
        }
    }

    fn set_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.ram.data[(address % 2048) as usize] = value,
            _ => todo!(),
        }
    }
}
