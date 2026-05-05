use crate::{bus::Bus, cpu::CPU};

pub mod arithmetic;
pub mod opcode;
pub mod stack;

type Instruction = fn(&mut CPU, &mut dyn Bus, Operand);

pub enum Operand {
    Address(u16),
    Value(u8),
    Accumulator,
    None, // This might be useless
}

impl Operand {
    pub fn read(&self, cpu: &CPU, bus: &dyn Bus) -> Option<u8> {
        match self {
            Operand::Address(address) => Some(bus.get_byte(*address)),
            Operand::Value(value) => Some(*value),
            Operand::Accumulator => Some(cpu.ac),
            Operand::None => None,
        }
    }

    pub fn write(&self, cpu: &mut CPU, bus: &mut dyn Bus, value: u8) {
        match self {
            Operand::Address(address) => bus.set_byte(*address, value),
            Operand::Value(_) => panic!("cannot write to immediate value"),
            Operand::Accumulator => cpu.ac = value,
            Operand::None => panic!(),
        }
    }
}
