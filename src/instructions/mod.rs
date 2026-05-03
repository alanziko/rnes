use crate::bus::Bus;
use crate::cpu::CPU;

pub mod arithmetic;
pub mod opcode;

type Instruction = fn(&mut CPU, &mut dyn Bus, Operand);

pub enum Operand {
    Address(u16),
    Value(u8),
    None,
}

impl Operand {
    pub fn fetch(self, bus: &dyn Bus) -> Option<u8> {
        match self {
            Operand::Address(address) => Some(bus.get_byte(address)),
            Operand::Value(value) => Some(value),
            Operand::None => None,
        }
    }
}
