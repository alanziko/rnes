use rnes_macros::opcode;

use crate::{
    addressing::AddressingMode::*,
    bus::Bus,
    cpu::{CPU, StatusRegister},
    instructions::Operand,
    instructions::opcode::{CyclePenalty::*, Opcode},
};

#[opcode(0x18, cycles = 2, mode = Implied)]
pub fn clear_carry(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.sr.remove(StatusRegister::Carry);
}

#[opcode(0xD8, cycles = 2, mode = Implied)]
pub fn clear_decimal(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.sr.remove(StatusRegister::Decimal);
}

#[opcode(0x58, cycles = 2, mode = Implied)]
pub fn clear_interrupt(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.sr.remove(StatusRegister::Interrupt);
}

#[opcode(0xB8, cycles = 2, mode = Implied)]
pub fn clear_overflow(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.sr.remove(StatusRegister::Overflow);
}

#[opcode(0x38, cycles = 2, mode = Implied)]
pub fn set_carry(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.sr.insert(StatusRegister::Carry);
}

#[opcode(0xF8, cycles = 2, mode = Implied)]
pub fn set_decimal(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.sr.insert(StatusRegister::Decimal);
}

#[opcode(0x78, cycles = 2, mode = Implied)]
pub fn set_interrupt(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.sr.insert(StatusRegister::Interrupt);
}

#[cfg(test)]
mod tests {
    use crate::bus::Memory;

    use super::*;

    #[test]
    fn manipulate_carry() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();

        set_carry(&mut cpu, &mut bus, Operand::None);
        assert!(cpu.sr.contains(StatusRegister::Carry));
        clear_carry(&mut cpu, &mut bus, Operand::None);
        assert!(!cpu.sr.contains(StatusRegister::Carry));
    }

    #[test]
    fn manipulate_decimal() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();

        set_decimal(&mut cpu, &mut bus, Operand::None);
        assert!(cpu.sr.contains(StatusRegister::Decimal));
        clear_decimal(&mut cpu, &mut bus, Operand::None);
        assert!(!cpu.sr.contains(StatusRegister::Decimal));
    }

    #[test]
    fn manipulate_interrupt() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();

        set_interrupt(&mut cpu, &mut bus, Operand::None);
        assert!(cpu.sr.contains(StatusRegister::Interrupt));
        clear_interrupt(&mut cpu, &mut bus, Operand::None);
        assert!(!cpu.sr.contains(StatusRegister::Interrupt));
    }

    #[test]
    fn manipulate_overflow() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();

        cpu.sr.insert(StatusRegister::Overflow);

        clear_overflow(&mut cpu, &mut bus, Operand::None);
        assert!(!cpu.sr.contains(StatusRegister::Overflow));
    }
}
