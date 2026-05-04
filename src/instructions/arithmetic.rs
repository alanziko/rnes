use rnes_macros::opcode;

use crate::{
    addressing::AddressingMode::*,
    bus::Bus,
    cpu::{CPU, StatusRegister},
    instructions::Operand,
    instructions::opcode::CyclePenalty::*,
    instructions::opcode::Opcode,
};

#[opcode(0x69, cycles = 2, mode = Immediate)]
#[opcode(0x65, cycles = 3, mode = ZeroPage)]
#[opcode(0x75, cycles = 4, mode = ZeroX)]
#[opcode(0x6D, cycles = 4, mode = Absolute)]
#[opcode(0x7D, cycles = 4, mode = AbsoluteX, penalty = BoundaryCrossed)] //*
#[opcode(0x79, cycles = 4, mode = AbsoluteY, penalty = BoundaryCrossed)] //*
#[opcode(0x61, cycles = 6, mode = IndirectX)]
#[opcode(0x71, cycles = 5, mode = IndirectY, penalty = BoundaryCrossed)] //*
pub fn add_with_carry(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let a = cpu.ac;
    let m = operand.fetch(bus).unwrap();
    let c = cpu.sr.contains(StatusRegister::Carry);
    let sum = a as u16 + m as u16 + c as u16;

    let result = sum as u8;

    cpu.sr.set_flag(StatusRegister::Carry, sum > 0xFF);
    cpu.sr.set_flag(StatusRegister::Zero, result == 0);
    let negative = (result & 0x80) != 0;
    cpu.sr.set_flag(StatusRegister::Negative, negative);
    let overflow = ((a ^ result) & (m ^ result) & 0x80) != 0;
    cpu.sr.set_flag(StatusRegister::Overflow, overflow);

    cpu.ac = result;
}

pub fn subtract_with_borrow(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let a = cpu.ac;
    let m = operand.fetch(bus).unwrap();
    let c = 1 - cpu.sr.contains(StatusRegister::Carry) as u8;
    let sum = (a as u16).wrapping_sub(m as u16).wrapping_sub(c as u16);

    let result = sum as u8;

    let carry = a as u16 >= (m as u16 + c as u16);
    cpu.sr.set_flag(StatusRegister::Carry, carry);
    cpu.sr.set_flag(StatusRegister::Zero, result == 0);
    let negative = (result & 0x80) != 0;
    cpu.sr.set_flag(StatusRegister::Negative, negative);
    let overflow = ((a ^ result) & (a ^ m) & 0x80) != 0;
    cpu.sr.set_flag(StatusRegister::Overflow, overflow);

    cpu.ac = result;
}

#[cfg(test)]
mod tests {
    use crate::bus::Memory;

    use super::*;

    #[test]
    fn add_with_carry_carry() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        bus.set_byte(0x0000, 30);
        cpu.ac = 0;
        cpu.sr.insert(StatusRegister::Carry);

        add_with_carry(&mut cpu, &mut bus, Operand::Address(0x0000));
        assert_eq!(cpu.ac, 31);
    }

    #[test]
    fn add_with_carry_no_carry() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();

        cpu.ac = 0;

        add_with_carry(&mut cpu, &mut bus, Operand::Value(10));
        assert_eq!(cpu.ac, 10);
        assert!(!cpu.sr.contains(StatusRegister::Carry));
    }

    #[test]
    fn add_with_carry_overflow() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 100;

        add_with_carry(&mut cpu, &mut bus, Operand::Value(50));
        assert!(cpu.sr.contains(StatusRegister::Overflow));
    }

    #[test]
    fn add_with_carry_negative() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 100;

        add_with_carry(&mut cpu, &mut bus, Operand::Value(50));
        assert!(cpu.sr.contains(StatusRegister::Negative));
    }

    #[test]
    fn add_with_carry_zero() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();

        cpu.ac = 0;
        add_with_carry(&mut cpu, &mut bus, Operand::Value(0));
        assert!(cpu.sr.contains(StatusRegister::Zero));
    }

    #[test]
    fn sub_with_carry_zero() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 10;
        cpu.sr.insert(StatusRegister::Carry);

        subtract_with_borrow(&mut cpu, &mut bus, Operand::Value(10));

        assert!(cpu.sr.contains(StatusRegister::Zero));
    }

    #[test]
    fn sub_with_carry_borrow() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 0;
        cpu.sr.insert(StatusRegister::Carry);

        subtract_with_borrow(&mut cpu, &mut bus, Operand::Value(1));

        assert_eq!(cpu.ac, 255);
        assert!(!cpu.sr.contains(StatusRegister::Carry));
    }

    #[test]
    fn sub_with_carry_no_borrow() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 10;
        cpu.sr.insert(StatusRegister::Carry);

        subtract_with_borrow(&mut cpu, &mut bus, Operand::Value(0));

        assert!(cpu.sr.contains(StatusRegister::Carry));
    }

    #[test]
    fn sub_with_carry_edge_borrow() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 0;
        cpu.sr.insert(StatusRegister::Carry);

        subtract_with_borrow(&mut cpu, &mut bus, Operand::Value(0));

        assert!(cpu.sr.contains(StatusRegister::Carry));
    }

    #[test]
    fn sub_with_carry_overflow() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 127;
        cpu.sr.insert(StatusRegister::Carry);

        subtract_with_borrow(&mut cpu, &mut bus, Operand::Value(255));
        assert!(cpu.sr.contains(StatusRegister::Overflow));
        assert!(cpu.sr.contains(StatusRegister::Negative));
    }

    #[test]
    fn sub_with_carry_negative() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();
        cpu.ac = 20;

        subtract_with_borrow(&mut cpu, &mut bus, Operand::Value(40));
        assert!(cpu.sr.contains(StatusRegister::Negative));
    }
}
