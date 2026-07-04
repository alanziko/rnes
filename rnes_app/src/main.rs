use rnes_core::bus::Bus;
use rnes_core::cpu::CPU;
use rnes_core::cpu::bus::CPUBus;
use std::fs;

fn main() {
    let mut cpu = CPU::new();
    let mut bus = CPUBus::new();

    let bytes = fs::read("code.bin").unwrap();

    let mut addr = 0x0600;

    for byte in bytes {
        bus.set_byte(addr, byte);
        addr += 1;
    }

    for _ in 0..20 {
        cpu.step(&mut bus);
    }

    println!("{}", cpu.ac);
}
