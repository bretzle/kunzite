//! The cpu

mod decode;
mod instruction;

/// The cpu
pub struct Cpu {
	rom: Vec<u8>,
	pc: usize,
}

impl Cpu {}
