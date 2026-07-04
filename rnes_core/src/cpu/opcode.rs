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
            code,
            instruction,
            mode,
            cycles,
            cycle_penalty,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CyclePenalty {
    BoundaryCrossed,
    Branch,
    None,
}

static OPCODES: OnceLock<[Option<&'static Opcode>; 256]> = OnceLock::new();

inventory::collect!(Opcode);

pub fn get_opcodes_lookup() -> &'static [Option<&'static Opcode>; 256] {
    OPCODES.get_or_init(|| {
        inventory::iter::<Opcode>
            .into_iter()
            .fold([None; _], |mut acc, op| {
                acc[op.code as usize] = Some(op);
                acc
            })
    })
}
