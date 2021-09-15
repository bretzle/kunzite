//!

use std::path::Path;

use crate::{cpu::Cpu, display::Display};
use color_eyre::Result;

/// Brings all the components into a single package
pub struct Gb {
	/// the cpu
	pub cpu: Cpu,
	_display: Display,
}

impl Gb {
	/// Create a new Gameboy instance
	pub fn new() -> Self {
		Self {
			cpu: Cpu::default(),
			_display: Display::new(),
		}
	}

	/// Insert a rom into the gameboy
	pub fn insert_rom<P: AsRef<Path> + Clone>(&mut self, path: P) -> Result<()> {
		self.cpu.memory.cartridge.insert_rom(path)
	}

	/// fully execute the next instruction
	pub fn step(&mut self) {
		self.cpu.step()
	}
}
