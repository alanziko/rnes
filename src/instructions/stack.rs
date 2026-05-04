use rnes_macros::opcode;

use crate::{
    addressing::AddressingMode::*,
    bus::Bus,
    cpu::{CPU, StatusRegister},
    instructions::{
        Operand,
        opcode::{CyclePenalty::*, Opcode},
    },
};

const STACK: u16 = 0x0100;

#[opcode(0x48, cycles = 3, mode = Implied)]
pub fn push_accumulator(cpu: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    bus.set_byte(cpu.sp as u16 | STACK, cpu.ac);
    cpu.sp = cpu.sp.wrapping_sub(1);
}

#[opcode(0x08, cycles = 3, mode = Implied)]
pub fn push_status_register(cpu: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    let mut sr = cpu.sr.clone();
    sr.insert(StatusRegister::Break);
    sr.insert(StatusRegister::Ignored);

    bus.set_byte(cpu.sp as u16 | STACK, sr.bits());
    cpu.sp = cpu.sp.wrapping_sub(1);
}

#[opcode(0x68, cycles = 4, mode = Implied)]
pub fn pull_accumulator(cpu: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    cpu.sp = cpu.sp.wrapping_add(1);
    cpu.ac = bus.get_byte(cpu.sp as u16 | STACK);

    cpu.sr.set_flag(StatusRegister::Zero, cpu.ac == 0);
    let negative = (cpu.ac & 0x80) != 0;
    cpu.sr.set_flag(StatusRegister::Negative, negative);
}

#[opcode(0x28, cycles = 4, mode = Implied)]
pub fn pull_status_register(cpu: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    cpu.sp = cpu.sp.wrapping_add(1);
    let mut sr = StatusRegister::from_bits_truncate(bus.get_byte(cpu.sp as u16 | STACK));

    sr.insert(StatusRegister::Ignored);
    sr.insert(StatusRegister::Break); // !

    cpu.sr = sr;
}
