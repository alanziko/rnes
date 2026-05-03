mod addressing;
mod bus;
mod cpu;
mod opcode;

use crate::addressing::AddressingMode;
use crate::bus::{Bus, Memory};
use crate::cpu::CPU;
use crate::opcode::Opcode;
use crate::opcode::instructions::Operand;

fn instruction(state: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    state.pc = 67;
    bus.set_byte(0x0000, 67);
}

fn main() {
    let mut cpu = CPU::default();
    let mut ram = Memory::new();

    let a = Opcode::new(instruction, AddressingMode::Implied, 2);
    (a.instruction)(&mut cpu, &mut ram, Operand::None);

    println!("{}", cpu.pc);
}
