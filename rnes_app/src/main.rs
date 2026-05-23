use rnes_core::cpu::addressing::AddressingMode;
use rnes_core::bus::{Bus, DebugBus};
use rnes_core::cpu::CPU;
use rnes_core::cpu::instructions::Operand;
use rnes_core::cpu::instructions::opcode::{CyclePenalty, Opcode};

fn instruction(state: &mut CPU, bus: &mut dyn Bus, _: Operand) {
    state.pc = 67;
    bus.set_byte(0x0000, 67);
}

fn main() {


    let mut cpu = CPU::default();
    let mut ram = DebugBus::new();

    let a = Opcode::new(
        1,
        instruction,
        AddressingMode::Implied,
        2,
        CyclePenalty::None,
    );
    (a.instruction)(&mut cpu, &mut ram, Operand::None);

    println!("{}", cpu.pc);
}
