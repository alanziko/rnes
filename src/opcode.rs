use crate::addressing::AddressingMode;

pub struct Opcode {
    pub instruction: fn(),
    mode: AddressingMode,
    cycles: u8,
}

impl Opcode {
    pub fn new(instruction: fn(), mode: AddressingMode, cycles: u8) -> Self {
        Self {
            instruction: instruction,
            mode: mode,
            cycles: cycles,
        }
    }
}
