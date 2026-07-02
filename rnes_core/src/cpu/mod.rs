pub mod addressing;
pub mod bus;
pub mod instructions;
pub mod opcode;
pub mod operand;

use bitflags::bitflags;

use crate::{
    bus::Bus,
    cpu::{
        addressing::AddressingMode::*,
        opcode::{Opcode, get_opcodes_lookup},
        operand::Operand,
    },
};

// TODO
// implement correct default values

#[derive(Default, Clone)]
pub struct CPU {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: StatusRegister,
    pub sp: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0x0600, // change to reset vector
            ac: 0,
            x: 0,
            y: 0,
            sr: StatusRegister::default(),
            sp: 0,
        }
    }

    pub fn step(&mut self, bus: &mut dyn Bus) {
        let opcode = bus.get_byte(self.pc);
        let opcode: &Opcode = get_opcodes_lookup()[opcode as usize].unwrap();

        self.pc += 1;
        let operand: Operand = match opcode.mode {
            Accumulator => Operand::Accumulator,
            Absolute => {
                let operand = Operand::Address(bus.get_word(self.pc));
                self.pc += 2;
                operand
            }
            AbsoluteX => {
                let operand = Operand::Address(bus.get_word(self.pc) + self.x as u16);
                self.pc += 2;
                operand
            }
            AbsoluteY => {
                let operand = Operand::Address(bus.get_word(self.pc) + self.y as u16);
                self.pc += 2;
                operand
            }
            Immediate => {
                let operand = Operand::Value(bus.get_byte(self.pc));
                self.pc += 1;
                operand
            }
            Implied => Operand::None,
            Indirect => {
                let ptr = bus.get_word(self.pc);
                // This implements JMP bug where page isn't incremented
                let low = bus.get_byte(ptr);
                let high = bus.get_byte(((ptr + 1) & 0x00FF) | (ptr & 0xFF00));

                let address = u16::from_le_bytes([low, high]);
                self.pc += 2;
                Operand::Address(address)
            }
            IndirectX => {
                let ptr = bus.get_byte(self.pc) + self.x;
                let low = bus.get_byte(ptr as u16);
                let high = bus.get_byte(ptr.wrapping_add(1) as u16);

                let address = u16::from_le_bytes([low, high]);
                self.pc += 1;
                Operand::Address(address)
            }
            IndirectY => {
                let ptr = bus.get_byte(self.pc);
                let low = bus.get_byte(ptr as u16);
                let high = bus.get_byte(ptr.wrapping_add(1) as u16);
                let address = u16::from_le_bytes([low, high]).wrapping_add(self.y as u16);
                self.pc += 1;
                Operand::Address(address)
            }
            // potential bug here
            Relative => {
                let offset = bus.get_byte(self.pc) as i8;
                let address = self.pc as i16 + offset as i16;
                Operand::Address(address as u16)
            }
            ZeroPage => {
                let address = bus.get_byte(self.pc) as u16;
                let operand = Operand::Address(address & 0xFF);
                self.pc += 1;
                operand
            }
            ZeroX => {
                let mut address = bus.get_byte(self.pc) as u16;
                address = address.wrapping_add(self.x as u16);
                let operand = Operand::Address(address & 0xFF);
                self.pc += 1;
                operand
            }
            ZeroY => {
                let mut address = bus.get_byte(self.pc) as u16;
                address = address.wrapping_add(self.y as u16);
                let operand = Operand::Address(address.wrapping_add(self.y as u16) & 0xFF);
                self.pc += 1;
                operand
            }
        };
        (opcode.instruction)(self, bus, operand);
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    pub struct StatusRegister: u8 {
        const Negative  = 0b10000000;
        const Overflow  = 0b01000000;
        const Ignored   = 0b00100000;
        const Break     = 0b00010000;
        const Decimal   = 0b00001000;
        const Interrupt = 0b00000100;
        const Zero      = 0b00000010;
        const Carry     = 0b00000001;
    }
}

impl Default for StatusRegister {
    fn default() -> Self {
        StatusRegister::empty()
    }
}
