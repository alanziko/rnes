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
