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

#[opcode(0xA9, cycles = 2, mode = Immediate)]
#[opcode(0xA5, cycles = 3, mode = Immediate)]
#[opcode(0xB5, cycles = 4, mode = Immediate)]
#[opcode(0xAD, cycles = 4, mode = Immediate)]
#[opcode(0xBD, cycles = 4, mode = Immediate, penalty = BoundaryCrossed)]
#[opcode(0xB9, cycles = 4, mode = Immediate, penalty = BoundaryCrossed)]
#[opcode(0xA1, cycles = 6, mode = Immediate)]
#[opcode(0xB1, cycles = 5, mode = Immediate, penalty = BoundaryCrossed)]
pub fn load_accumulator(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();

    cpu.ac = value;

    cpu.sr.set(StatusRegister::Zero, value == 0);
    let negative = (value & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x85, cycles = 3, mode = ZeroPage)]
#[opcode(0x95, cycles = 4, mode = ZeroX)]
#[opcode(0x8D, cycles = 4, mode = Absolute)]
#[opcode(0x9D, cycles = 5, mode = AbsoluteX)]
#[opcode(0x99, cycles = 5, mode = AbsoluteY)]
#[opcode(0x81, cycles = 6, mode = IndirectX)]
#[opcode(0x91, cycles = 6, mode = IndirectY)]
pub fn store_accumulator(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    operand.write(cpu, bus, cpu.ac);
}

#[opcode(0xA2, cycles = 2, mode = Immediate)]
#[opcode(0xA6, cycles = 3, mode = ZeroPage)]
#[opcode(0xB6, cycles = 4, mode = ZeroY)]
#[opcode(0xAE, cycles = 4, mode = Absolute)]
#[opcode(0xBE, cycles = 4, mode = AbsoluteY, penalty = BoundaryCrossed)]
pub fn load_x(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();

    cpu.x = value;

    cpu.sr.set(StatusRegister::Zero, value == 0);
    let negative = (value & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x86, cycles = 3, mode = ZeroPage)]
#[opcode(0x96, cycles = 4, mode = ZeroY)]
#[opcode(0x8E, cycles = 4, mode = Absolute)]
pub fn store_x(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    operand.write(cpu, bus, cpu.x);
}

#[opcode(0xA0, cycles = 2, mode = Immediate)]
#[opcode(0xA4, cycles = 3, mode = ZeroPage)]
#[opcode(0xB4, cycles = 4, mode = ZeroX)]
#[opcode(0xAC, cycles = 4, mode = Absolute)]
#[opcode(0xBC, cycles = 4, mode = AbsoluteX, penalty = BoundaryCrossed)]
pub fn load_y(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let value = operand.read(cpu, bus).unwrap();

    cpu.y = value;

    cpu.sr.set(StatusRegister::Zero, value == 0);
    let negative = (value & 0x80) != 0;
    cpu.sr.set(StatusRegister::Negative, negative);
}

#[opcode(0x84, cycles = 3, mode = ZeroPage)]
#[opcode(0x94, cycles = 4, mode = ZeroX)]
#[opcode(0x8C, cycles = 4, mode = Absolute)]
pub fn store_y(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    operand.write(cpu, bus, cpu.y);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::DebugBus;

    #[test]
    pub fn load_store_accumulator() {
        let mut cpu = CPU::default();
        let mut bus = DebugBus::new();

        let address = 0x0000;
        let operand = Operand::Address(address);
        cpu.ac = 10;

        store_accumulator(&mut cpu, &mut bus, operand);
        assert_eq!(bus.get_byte(address), 10);

        cpu.ac = 2;

        load_accumulator(&mut cpu, &mut bus, operand);
        assert_eq!(cpu.ac, 10);
    }

    #[test]
    pub fn load_store_x() {
        let mut cpu = CPU::default();
        let mut bus = DebugBus::new();

        let address = 0x0000;
        let operand = Operand::Address(address);
        cpu.x = 10;

        store_x(&mut cpu, &mut bus, operand);
        assert_eq!(bus.get_byte(address), 10);

        cpu.x = 2;

        load_x(&mut cpu, &mut bus, operand);
        assert_eq!(cpu.x, 10);
    }

    #[test]
    pub fn load_store_y() {
        let mut cpu = CPU::default();
        let mut bus = DebugBus::new();

        let address = 0x0000;
        let operand = Operand::Address(address);
        cpu.y = 10;

        store_y(&mut cpu, &mut bus, operand);
        assert_eq!(bus.get_byte(address), 10);

        cpu.y = 2;

        load_y(&mut cpu, &mut bus, operand);
        assert_eq!(cpu.y, 10);
    }
}
