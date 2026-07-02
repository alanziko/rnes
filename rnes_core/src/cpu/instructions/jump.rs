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

const STACK: u16 = 0x0100;

#[opcode(0x4C, cycles = 3, mode = Absolute)]
#[opcode(0x6C, cycles = 5, mode = Indirect)]
pub fn jump(cpu: &mut CPU, _: &mut dyn Bus, operand: Operand) {
    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0x20, cycles = 6, mode = Absolute)]
pub fn jump_to_subroutine(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let low = (cpu.pc.wrapping_add(2) & 0xFF) as u8;
    let high = (cpu.pc.wrapping_add(2) >> 8) as u8;

    bus.set_byte(cpu.sp as u16 | STACK, high);
    cpu.sp = cpu.sp.wrapping_sub(1);

    bus.set_byte(cpu.sp as u16 | STACK, low);
    cpu.sp = cpu.sp.wrapping_sub(1);

    cpu.pc = operand.read_address().unwrap();
}

#[opcode(0x60, cycles = 6, mode = Implied)]
pub fn return_from_subroutine(cpu: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    cpu.sp = cpu.sp.wrapping_add(1);
    let low = bus.get_byte(cpu.sp as u16 | STACK);
    cpu.sp = cpu.sp.wrapping_add(1);
    let high = bus.get_byte(cpu.sp as u16 | STACK);

    let address = u16::from_le_bytes([low, high]);
    cpu.pc = address.wrapping_add(1);
}

// Implementing as Immediate because it skips one byte
#[opcode(0x00, cycles = 7, mode = Immediate)]
pub fn break_irq(cpu: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    let low = (cpu.pc.wrapping_add(2) & 0xFF) as u8;
    let high = (cpu.pc.wrapping_add(2) >> 8) as u8;

    bus.set_byte(cpu.sp as u16 | STACK, high);
    cpu.sp = cpu.sp.wrapping_sub(1);

    bus.set_byte(cpu.sp as u16 | STACK, low);
    cpu.sp = cpu.sp.wrapping_sub(1);

    let mut sr = cpu.sr;
    sr &= StatusRegister::from_bits_truncate(0b00110000);
    sr.insert(StatusRegister::Break);

    bus.set_byte(cpu.sp as u16 | STACK, sr.bits());
    cpu.sp = cpu.sp.wrapping_sub(1);

    cpu.sr.insert(StatusRegister::Interrupt);

    cpu.pc = 0xFFFE;
}

#[opcode(0x40, cycles = 6, mode = Implied)]
pub fn return_from_interrupt(cpu: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    cpu.sp = cpu.sp.wrapping_add(1);
    let mut sr = StatusRegister::from_bits_truncate(bus.get_byte(cpu.sp as u16 | STACK));

    sr &= StatusRegister::from_bits_truncate(0b11001111);
    cpu.sr = sr;

    cpu.sp = cpu.sp.wrapping_add(1);
    let low = bus.get_byte(cpu.sp as u16 | STACK);
    cpu.sp = cpu.sp.wrapping_add(1);
    let high = bus.get_byte(cpu.sp as u16 | STACK);

    let address = u16::from_le_bytes([low, high]);
    cpu.pc = address;
}
