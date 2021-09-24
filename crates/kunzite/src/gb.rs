//!

use crate::{
	cpu::Cpu,
	memory::{Cartridge, Memory},
};
use color_eyre::Result;
use std::{fs::File, io::Read, path::Path};

/// Brings all the components into a single package
pub struct Gb {
	/// the cpu
	pub cpu: Cpu,
	memory: Memory,
}

impl Gb {
	/// Insert a rom into the gameboy
	pub fn insert_rom<P: AsRef<Path>>(path: P) -> Result<Self> {
		let mut file = File::open(path)?;
		let mut buffer = vec![];
		let _ = file.read_to_end(&mut buffer)?;

		let cartridge = Cartridge::from_rom(buffer)?;
		let memory = Memory::from_cartridge(cartridge, false);

		Ok(Self {
			cpu: Cpu::new(false),
			memory,
		})
	}

	/// fully execute the next instruction
	pub fn step(&mut self) -> u8 {
		self.cpu.step(&mut self.memory) as u8
	}

	pub fn memory(&self) -> &Memory {
		&self.memory
	}
}
