use rnes_macros::opcode;

use crate::{
    bus::Bus,
    cpu::{
        CPU, StatusRegister,
        addressing::AddressingMode::*,
        instructions::{
            Operand,
            opcode::{CyclePenalty::*, Opcode},
        },
    },
};

#[opcode(0x0A, cycles = 2, mode = Accumulator)]
#[opcode(0x06, cycles = 5, mode = ZeroPage)]
#[opcode(0x16, cycles = 6, mode = ZeroX)]
#[opcode(0x0E, cycles = 6, mode = Absolute)]
#[opcode(0x1E, cycles = 7, mode = AbsoluteX)]
fn arithmetic_shift_left(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let mut value = operand.read(cpu, bus).unwrap();

    cpu.sr.set(StatusRegister::Carry, value & 0x80 != 0);
    value = value << 1;

    operand.write(cpu, bus, value);

    cpu.sr.set(StatusRegister::Zero, value == 0);
    cpu.sr.set(StatusRegister::Negative, value & 0x80 != 0);
}

#[opcode(0x4A, cycles = 2, mode = Accumulator)]
#[opcode(0x46, cycles = 5, mode = ZeroPage)]
#[opcode(0x56, cycles = 6, mode = ZeroX)]
#[opcode(0x4E, cycles = 6, mode = Absolute)]
#[opcode(0x5E, cycles = 7, mode = AbsoluteX)]
fn logical_shift_right(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let mut value = operand.read(cpu, bus).unwrap();

    cpu.sr.set(StatusRegister::Carry, value & 0x01 != 0);
    value = value >> 1;

    operand.write(cpu, bus, value);

    cpu.sr.set(StatusRegister::Zero, value == 0);
    cpu.sr.remove(StatusRegister::Negative);
}

#[opcode(0x2A, cycles = 2, mode = Accumulator)]
#[opcode(0x26, cycles = 5, mode = ZeroPage)]
#[opcode(0x36, cycles = 6, mode = ZeroX)]
#[opcode(0x2E, cycles = 6, mode = Absolute)]
#[opcode(0x3E, cycles = 7, mode = AbsoluteX)]
fn rotate_left(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let mut value = operand.read(cpu, bus).unwrap();

    let carry = cpu.sr.contains(StatusRegister::Carry);
    cpu.sr.set(StatusRegister::Carry, value & 0x80 != 0);
    value = value << 1;

    if carry {
        value = value | 0x01;
    }

    operand.write(cpu, bus, value);

    cpu.sr.set(StatusRegister::Zero, value == 0);
    cpu.sr.set(StatusRegister::Negative, value & 0x80 != 0)
}

#[opcode(0x6A, cycles = 2, mode = Accumulator)]
#[opcode(0x66, cycles = 5, mode = ZeroPage)]
#[opcode(0x76, cycles = 6, mode = ZeroX)]
#[opcode(0x6E, cycles = 6, mode = Absolute)]
#[opcode(0x7E, cycles = 7, mode = AbsoluteX)]
fn rotate_right(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let mut value = operand.read(cpu, bus).unwrap();

    let carry = cpu.sr.contains(StatusRegister::Carry);
    cpu.sr.set(StatusRegister::Carry, value & 0x01 != 0);
    value = value >> 1;

    if carry {
        value = value | 0x80;
    }

    operand.write(cpu, bus, value);

    cpu.sr.set(StatusRegister::Zero, value == 0);
    cpu.sr.set(StatusRegister::Negative, value & 0x80 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::DebugBus;

    #[test]
    fn shift() {
        let mut cpu = CPU::default();
        let mut bus = DebugBus::new();

        cpu.ac = 0b01111111;
        arithmetic_shift_left(&mut cpu, &mut bus, Operand::Accumulator);
        assert_eq!(cpu.ac, 0b11111110);

        logical_shift_right(&mut cpu, &mut bus, Operand::Accumulator);
        assert_eq!(cpu.ac, 0b01111111);
    }

    #[test]
    fn rotate() {
        let mut cpu = CPU::default();
        let mut bus = DebugBus::new();

        cpu.ac = 0b11111111;
        rotate_left(&mut cpu, &mut bus, Operand::Accumulator);
        assert_eq!(cpu.ac, 0b11111110);
        assert!(cpu.sr.contains(StatusRegister::Carry));

        rotate_right(&mut cpu, &mut bus, Operand::Accumulator);
        assert_eq!(cpu.ac, 0b11111111);
        assert!(!cpu.sr.contains(StatusRegister::Carry))
    }
}
