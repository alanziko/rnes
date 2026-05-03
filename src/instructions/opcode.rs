use crate::{addressing::AddressingMode, instructions::Instruction};

inventory::collect!(Opcode);

#[derive(Debug)]
pub struct Opcode {
    pub code: u8,
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub cycles: u8,
}

impl Opcode {
    pub fn new(code: u8, instruction: Instruction, mode: AddressingMode, cycles: u8) -> Self {
        Self {
            code: code,
            instruction: instruction,
            mode: mode,
            cycles: cycles,
        }
    }
}
