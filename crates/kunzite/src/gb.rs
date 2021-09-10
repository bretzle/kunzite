//!

use crate::{cpu::Cpu, display::Display, memory::Memory};
use color_eyre::Result;

/// Brings all the components into a single package
pub struct Gb {
	cpu: Cpu,
	memory: Memory,
	display: Display,
}

impl Gb {
	/// Create a new Gameboy instance
	pub fn new() -> Self {
		Self {
			cpu: Cpu::default(),
			memory: Memory::new(),
			display: Display::new(),
		}
	}

	/// Insert a rom into the gameboy
	pub fn insert_rom(&mut self, arg: &str) -> Result<()> {
		self.cpu.insert_rom(arg)
	}

	/// fully execute the next instruction
	pub fn step(&mut self) {
		self.cpu.step()
	}
}
