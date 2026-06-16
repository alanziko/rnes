use bitflags::Flags;
use rnes_macros::opcode;

use crate::{
    bus::Bus,
    cpu::addressing::AddressingMode::*,
    cpu::instructions::{
        Operand,
        opcode::{CyclePenalty::*, Opcode},
    },
    cpu::{CPU, StatusRegister},
};

#[opcode(0x90, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_carry_clear(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if cpu.sr.contains(StatusRegister::Carry) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0xB0, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_carry_set(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if !cpu.sr.contains(StatusRegister::Carry) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0xF0, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_equal(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if !cpu.sr.contains(StatusRegister::Zero) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0xD0, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_not_equal(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if cpu.sr.contains(StatusRegister::Zero) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0x10, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_plus(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if cpu.sr.contains(StatusRegister::Negative) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0x30, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_minus(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if !cpu.sr.contains(StatusRegister::Negative) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0x50, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_overflow_clear(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if cpu.sr.contains(StatusRegister::Overflow) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0x70, cycles = 2, mode = Relative, penalty = Branch)]
pub fn branch_if_overflow_set(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    if !cpu.sr.contains(StatusRegister::Overflow) {
        return;
    }

    cpu.pc = operand.read_address().unwrap();
}
