use crate::{
    bus::Bus,
    cpu::{CPU, StatusRegister},
};

pub enum Operand {
    Address(u16),
    Value(u8),
    None,
}

impl Operand {
    pub fn fetch(self, bus: &dyn Bus) -> Option<u8> {
        match self {
            Operand::Address(address) => Some(bus.get_byte(address)),
            Operand::Value(value) => Some(value),
            Operand::None => None,
        }
    }
}

pub fn add_with_carry(cpu: &mut CPU, bus: &mut dyn Bus, operand: Operand) {
    let a = cpu.ac;
    let m = operand.fetch(bus).unwrap();
    let c = cpu.sr.contains(StatusRegister::Carry);
    let sum = a as u16 + m as u16 + c as u16;

    let result = sum as u8;

    cpu.sr.set_flag(StatusRegister::Carry, sum > 0xFF);
    cpu.sr.set_flag(StatusRegister::Zero, result == 0);
    let negative = (result & 0x80) != 0;
    cpu.sr.set_flag(StatusRegister::Negative, negative);
    let overflow = (a ^ result) & (m ^ result) & 0x80 != 0;
    cpu.sr.set_flag(StatusRegister::Overflow, overflow);

    cpu.ac = result;
}

#[cfg(test)]
mod tests {
    use crate::bus::Memory;

    use super::*;

    #[test]
    fn test_add_with_carry() {
        let mut cpu = CPU::default();
        let mut bus = Memory::new();

        bus.set_byte(0x0000, 30);

        cpu.ac = 0;

        // Carry
        add_with_carry(&mut cpu, &mut bus, Operand::Address(0x0000));
        assert_eq!(cpu.ac, 30);

        add_with_carry(&mut cpu, &mut bus, Operand::Value(255));
        assert_eq!(cpu.ac, 29);
        assert!(cpu.sr.contains(StatusRegister::Carry));

        add_with_carry(&mut cpu, &mut bus, Operand::Value(0));
        assert_eq!(cpu.ac, 30);

        // Oveflow
        cpu.ac = 100;

        add_with_carry(&mut cpu, &mut bus, Operand::Value(50));
        assert!(cpu.sr.contains(StatusRegister::Overflow));

        // Negative
        assert!(cpu.sr.contains(StatusRegister::Negative));

        // Zero
        cpu.ac = 0;
        cpu.sr.set_flag(StatusRegister::Carry, false);

        add_with_carry(&mut cpu, &mut bus, Operand::Value(0));
        assert!(cpu.sr.contains(StatusRegister::Zero));
    }
}
