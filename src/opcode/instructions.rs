use std::collections::btree_map::Values;

use crate::{
    bus::Bus,
    cpu::{CPU, StatusRegister},
};

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

pub fn add_with_carry(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let operand = operand.fetch(bus).unwrap();
    let (sum, carry) = cpu.ac.overflowing_add(operand);
    cpu.ac = sum;
    cpu.sr.set_flag(StatusRegister::Carry, carry);
    // Not done
}
