use rnes_macros::opcode;

use crate::{
    bus::Bus,
    cpu::{
        CPU, StatusRegister,
        addressing::AddressingMode::*,
        instructions::Operand,
        instructions::opcode::{CyclePenalty::*, Opcode},
    },
};

#[opcode(0xAA, cycles = 2, mode = Implied)]
pub fn transfer_a_to_x(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.x = cpu.ac;

    cpu.sr.set(StatusRegister::Zero, cpu.x == 0);
    let negative = (cpu.x & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x8A, cycles = 2, mode = Implied)]
pub fn transfer_x_to_a(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.ac = cpu.x;

    cpu.sr.set(StatusRegister::Zero, cpu.ac == 0);
    let negative = (cpu.ac & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0xA8, cycles = 2, mode = Implied)]
pub fn transfer_a_to_y(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.y = cpu.ac;

    cpu.sr.set(StatusRegister::Zero, cpu.y == 0);
    let negative = (cpu.y & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x98, cycles = 2, mode = Implied)]
pub fn transfer_y_to_a(cpu: &mut CPU, _: &mut dyn Bus, _: Operand) {
    cpu.ac = cpu.y;

    cpu.sr.set(StatusRegister::Zero, cpu.ac == 0);
    let negative = (cpu.ac & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::DebugBus;

    #[test]
    fn transfer_x() {
        let mut cpu = CPU::default();
        let mut bus = DebugBus::new();

        cpu.ac = 10;
        transfer_a_to_x(&mut cpu, &mut bus, Operand::None);
        assert_eq!(cpu.ac, cpu.x);

        cpu.ac = 0;
        transfer_x_to_a(&mut cpu, &mut bus, Operand::None);
        assert_eq!(cpu.ac, 10);
    }

    #[test]
    fn transfer_y() {
        let mut cpu = CPU::default();
        let mut bus = DebugBus::new();

        cpu.ac = 10;
        transfer_a_to_y(&mut cpu, &mut bus, Operand::None);
        assert_eq!(cpu.ac, cpu.y);

        cpu.ac = 0;
        transfer_y_to_a(&mut cpu, &mut bus, Operand::None);
        assert_eq!(cpu.ac, 10);
    }
}
