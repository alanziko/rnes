mod addressing;
mod bus;
mod cpu;
mod instructions;

use crate::addressing::AddressingMode;
use crate::bus::{Bus, Memory};
use crate::cpu::CPU;
use crate::instructions::Operand;
use crate::instructions::opcode::Opcode;

fn instruction(state: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    state.pc = 67;
    bus.set_byte(0x0000, 67);
}

fn main() {
    let mut cpu = CPU::default();
    let mut ram = Memory::new();

    let a = Opcode::new(1, instruction, AddressingMode::Implied, 2);
    (a.instruction)(&mut cpu, &mut ram, Operand::None);

    println!("{}", cpu.pc);

    for opcode in inventory::iter::<Opcode>() {
        if opcode.code == 0x69 {
            (opcode.instruction)(&mut cpu, &mut ram, Operand::Value(10));
            println!("{}", cpu.ac);
        }
    }
}
