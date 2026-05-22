pub trait Bus {
    fn get_byte(&self, address: u16) -> u8;
    fn set_byte(&mut self, address: u16, value: u8);

    fn get_word(&self, address: u16) -> u16 {
        let low = self.get_byte(address);
        let high = self.get_byte(address.wrapping_add(1));
        u16::from_le_bytes([low, high])
    }

    fn set_word(&mut self, address: u16, value: u16) {
        let [low, high] = value.to_le_bytes();
        self.set_byte(address, low);
        self.set_byte(address, high);
    }
}

pub struct DebugMemory {
    pub data: [u8; 65536],
}

impl DebugMemory {
    pub fn new() -> Self {
        Self { data: [0u8; 65536] }
    }
}

struct RAM {
    pub data: [u8; 2048],
}

impl RAM {
    pub fn new() -> Self {
        Self { data: [0u8; 2048] }
    }
}

pub struct MainBus {
    ram: RAM,
}

impl MainBus {
    pub fn new() -> Self {
        Self { ram: RAM::new() }
    }
}

impl Bus for MainBus {
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

pub struct DebugBus {
    memory: DebugMemory,
}

impl DebugBus {
    pub fn new() -> Self {
        Self {
            memory: DebugMemory::new(),
        }
    }
}

impl Bus for DebugBus {
    fn get_byte(&self, address: u16) -> u8 {
        self.memory.data[address as usize]
    }
    fn set_byte(&mut self, address: u16, value: u8) {
        self.memory.data[address as usize] = value;
    }
}
