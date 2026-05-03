use crate::addressing::AddressingMode;
use crate::bus::Bus;
use crate::cpu::CPU;

type Instruction = fn(&mut CPU, &mut dyn Bus);

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
