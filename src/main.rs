mod addressing;
mod bus;
mod cpu;
mod opcode;

use crate::addressing::AddressingMode;
use crate::cpu::CPU;
use crate::opcode::Opcode;

fn instruction(state: &mut CPU) {
    state.pc = 67;
}

fn main() {
    let mut cpu = CPU::default();

    let a = Opcode::new(instruction, AddressingMode::Implied, 2);
    (a.instruction)(&mut cpu);

    println!("{}", cpu.pc);
}
