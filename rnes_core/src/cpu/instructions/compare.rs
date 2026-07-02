use rnes_macros::opcode;

use crate::{
    bus::Bus,
    cpu::{
        CPU, StatusRegister,
        addressing::AddressingMode::*,
        instructions::Operand,
        opcode::{CyclePenalty::*, Opcode},
    },
};

#[opcode(0xC9, cycles = 2, mode = Immediate)]
#[opcode(0xC5, cycles = 3, mode = ZeroPage)]
#[opcode(0xD5, cycles = 4, mode = ZeroX)]
#[opcode(0xCD, cycles = 4, mode = Absolute)]
#[opcode(0xDD, cycles = 4, mode = AbsoluteX, penalty = BoundaryCrossed)]
#[opcode(0xD9, cycles = 4, mode = AbsoluteY, penalty = BoundaryCrossed)]
#[opcode(0xC1, cycles = 6, mode = IndirectX)]
#[opcode(0xD1, cycles = 5, mode = IndirectY, penalty = BoundaryCrossed)]
pub fn compare_a(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();
    let result = cpu.ac.wrapping_sub(value);

    cpu.sr.set(StatusRegister::Carry, cpu.ac >= value);
    cpu.sr.set(StatusRegister::Zero, cpu.ac == value);
    cpu.sr.set(StatusRegister::Negative, (result & 0x80) != 0);
}

#[opcode(0xE0, cycles = 2, mode = Immediate)]
#[opcode(0xE4, cycles = 3, mode = ZeroPage)]
#[opcode(0xEC, cycles = 4, mode = Absolute)]
pub fn compare_x(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();
    let result = cpu.x.wrapping_sub(value);

    cpu.sr.set(StatusRegister::Carry, cpu.x >= value);
    cpu.sr.set(StatusRegister::Zero, cpu.x == value);
    cpu.sr.set(StatusRegister::Negative, (result & 0x80) != 0);
}

#[opcode(0xC0, cycles = 2, mode = Immediate)]
#[opcode(0xC4, cycles = 3, mode = ZeroPage)]
#[opcode(0xCC, cycles = 4, mode = Absolute)]
pub fn compare_y(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();
    let result = cpu.y.wrapping_sub(value);

    cpu.sr.set(StatusRegister::Carry, cpu.y >= value);
    cpu.sr.set(StatusRegister::Zero, cpu.y == value);
    cpu.sr.set(StatusRegister::Negative, (result & 0x80) != 0);
}
