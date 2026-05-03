use crate::addressing::AddressingMode;
use crate::cpu::CPU;

pub struct Opcode {
    pub instruction: fn(&mut CPU),
    mode: AddressingMode,
    cycles: u8,
}

impl Opcode {
    pub fn new(instruction: fn(&mut CPU), mode: AddressingMode, cycles: u8) -> Self {
        Self {
            instruction: instruction,
            mode: mode,
            cycles: cycles,
        }
    }
}
