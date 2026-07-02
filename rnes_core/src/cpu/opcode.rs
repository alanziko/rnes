use std::sync::OnceLock;

use crate::{cpu::addressing::AddressingMode, cpu::instructions::Instruction};

#[derive(Debug, Clone, Copy)]
pub struct Opcode {
    pub code: u8,
    pub instruction: Instruction,
    pub mode: AddressingMode,
    pub cycles: usize,
    pub cycle_penalty: CyclePenalty,
}

impl Opcode {
    pub fn new(
        code: u8,
        instruction: Instruction,
        mode: AddressingMode,
        cycles: usize,
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

#[derive(Debug, Clone, Copy)]
pub enum CyclePenalty {
    BoundaryCrossed,
    Branch,
    None,
}

static OPCODE_LOOKUP: OnceLock<[Option<&'static Opcode>; 256]> = OnceLock::new();

inventory::collect!(Opcode);

pub fn get_opcodes_lookup() -> &'static [Option<&'static Opcode>; 256] {
    OPCODE_LOOKUP.get_or_init(|| {
        let mut lookup_table = [None; 256];
        for op in inventory::iter::<Opcode> {
            lookup_table[op.code as usize] = Some(op)
        }
        lookup_table
    })
}
