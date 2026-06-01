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

#[opcode(0x29, cycles = 2, mode = Immediate)]
#[opcode(0x25, cycles = 3, mode = ZeroPage)]
#[opcode(0x35, cycles = 4, mode = ZeroX)]
#[opcode(0x2D, cycles = 4, mode = Absolute)]
#[opcode(0x3D, cycles = 4, mode = AbsoluteX, penalty = BoundaryCrossed)]
#[opcode(0x39, cycles = 4, mode = AbsoluteY, penalty = BoundaryCrossed)]
#[opcode(0x21, cycles = 6, mode = IndirectX)]
#[opcode(0x31, cycles = 5, mode = IndirectY, penalty = BoundaryCrossed)]
pub fn bitwise_and(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();
    cpu.ac &= value;

    cpu.sr.set(StatusRegister::Zero, value == 0);
    let negative = (value & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x09, cycles = 2, mode = Immediate)]
#[opcode(0x05, cycles = 3, mode = ZeroPage)]
#[opcode(0x15, cycles = 4, mode = ZeroX)]
#[opcode(0x0D, cycles = 4, mode = Absolute)]
#[opcode(0x1D, cycles = 4, mode = AbsoluteX, penalty = BoundaryCrossed)]
#[opcode(0x19, cycles = 4, mode = AbsoluteY, penalty = BoundaryCrossed)]
#[opcode(0x01, cycles = 6, mode = IndirectX)]
#[opcode(0x11, cycles = 5, mode = IndirectY, penalty = BoundaryCrossed)]
pub fn bitwise_or(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();
    cpu.ac |= value;

    cpu.sr.set(StatusRegister::Zero, value == 0);
    let negative = (value & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x49, cycles = 2, mode = Immediate)]
#[opcode(0x45, cycles = 3, mode = ZeroPage)]
#[opcode(0x55, cycles = 4, mode = ZeroX)]
#[opcode(0x4D, cycles = 4, mode = Absolute)]
#[opcode(0x5D, cycles = 4, mode = AbsoluteX, penalty = BoundaryCrossed)]
#[opcode(0x59, cycles = 4, mode = AbsoluteY, penalty = BoundaryCrossed)]
#[opcode(0x41, cycles = 6, mode = IndirectX)]
#[opcode(0x51, cycles = 5, mode = IndirectY, penalty = BoundaryCrossed)]
pub fn bitwise_xor(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();
    cpu.ac ^= value;

    cpu.sr.set(StatusRegister::Zero, value == 0);
    let negative = (value & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x24, cycles = 3, mode = ZeroPage)]
#[opcode(0x2C, cycles = 4, mode = Absolute)]
pub fn bit_test(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();
    let result = cpu.ac & value;

    cpu.sr.set(StatusRegister::Zero, result != 0);
    let negative = (value & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
    let overflow = (value & 0x20) != 0;
    cpu.sr.set(StatusRegister::Overflow, overflow);
}
