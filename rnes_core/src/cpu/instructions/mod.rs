use crate::{
    bus::Bus,
    cpu::{CPU, operand::Operand},
};

pub mod access;
pub mod arithmetic;
pub mod bitwise;
pub mod branch;
pub mod compare;
pub mod flag;
pub mod jump;
pub mod shift;
pub mod stack;
pub mod transfer;

pub type Instruction = fn(&mut CPU, &mut dyn Bus, Operand);
