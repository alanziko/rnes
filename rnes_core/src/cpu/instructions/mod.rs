use crate::{bus::Bus, cpu::CPU};

pub mod access;
pub mod arithmetic;
pub mod bitwise;
pub mod branch;
pub mod compare;
pub mod flag;
pub mod jump;
pub mod opcode;
pub mod shift;
pub mod stack;
pub mod transfer;

type Instruction = fn(&mut CPU, &mut dyn Bus, Operand);

#[derive(Copy, Clone)]
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

    pub fn read_address(&self) -> Option<u16> {
        if let Operand::Address(address) = self {
            Some(*address)
        } else {
            None
        }
    }
}
