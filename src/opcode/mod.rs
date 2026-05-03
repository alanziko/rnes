use crate::addressing::AddressingMode;
use crate::bus::Bus;
use crate::cpu::CPU;
use crate::opcode::instructions::Operand;

pub mod instructions;

type Instruction = fn(&mut CPU, &mut dyn Bus, Operand);

pub struct Opcode {
    pub instruction: Instruction,
    mode: AddressingMode,
    cycles: u8,
}

impl Opcode {
    pub fn new(instruction: Instruction, mode: AddressingMode, cycles: u8) -> Self {
        Self {
            instruction: instruction,
            mode: mode,
            cycles: cycles,
        }
    }
}
