mod addressing;
mod opcode;

use crate::addressing::AddressingMode;
use crate::opcode::Opcode;

fn instruction() {
    println!("Hello world!");
}

fn main() {
    let a = Opcode::new(instruction, AddressingMode::Implied, 2);
    (a.instruction)();
}
