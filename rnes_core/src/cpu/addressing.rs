use crate::{
    bus::Bus,
    cpu::{CPU, instructions::Operand},
};
use AddressingMode::*;

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroX,
    ZeroY,
}

impl AddressingMode {
    pub fn eval(&self, cpu: &mut CPU, bus: &mut dyn Bus, operand: u16) -> Operand {
        match self {
            Accumulator => Operand::Accumulator,
            Absolute => Operand::Address(operand),
            AbsoluteX => Operand::Address(operand + cpu.x as u16),
            AbsoluteY => Operand::Address(operand + cpu.y as u16),
            Immediate => Operand::Value(operand as u8),
            Implied => Operand::None,
            Indirect => {
                // This implements JMP bug where page isn't incremented
                let low = bus.get_byte(operand);
                let high = bus.get_byte(((operand + 1) & 0x00FF) | (operand & 0xFF00));

                let address = u16::from_le_bytes([low, high]);
                Operand::Address(address)
            }
            IndirectX => {
                let ptr = ((operand & 0xFF) + cpu.x as u16) as u8;
                let low = bus.get_byte(ptr as u16);
                let high = bus.get_byte(ptr.wrapping_add(1) as u16);

                let address = u16::from_le_bytes([low, high]);
                Operand::Address(address)
            }
            IndirectY => {
                let ptr = operand as u8;
                let low = bus.get_byte(ptr as u16);
                let high = bus.get_byte(ptr.wrapping_add(1) as u16);
                let address = u16::from_le_bytes([low, high]).wrapping_add(cpu.y as u16);
                Operand::Address(address)
            }
            Relative => {
                let offset = operand as i8;
                let address = cpu.pc as i16 + offset as i16;
                Operand::Address(address as u16)
            }
            ZeroPage => Operand::Address(operand & 0xFF),
            ZeroX => Operand::Address(operand.wrapping_add(cpu.x as u16) & 0xFF),
            ZeroY => Operand::Address(operand.wrapping_add(cpu.y as u16) & 0xFF),
        }
    }
}
