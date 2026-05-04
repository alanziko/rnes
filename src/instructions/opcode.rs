use crate::{addressing::AddressingMode, instructions::Instruction};

inventory::collect!(Opcode);

#[derive(Debug)]
pub struct Opcode {
    pub code: u8,
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub cycles: u8,
    pub cycle_penalty: CyclePenalty,
}

impl Opcode {
    pub fn new(
        code: u8,
        instruction: Instruction,
        mode: AddressingMode,
        cycles: u8,
        cycle_penalty: CyclePenalty,
    ) -> Self {
        Self {
            code: code,
            instruction: instruction,
            mode: mode,
            cycles: cycles,
            cycle_penalty: cycle_penalty,
        }
    }
}

#[derive(Debug)]
pub enum CyclePenalty {
    BoundaryCrossed,
    Branch,
    None,
}
