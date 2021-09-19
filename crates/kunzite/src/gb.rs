//!

use std::path::Path;

use crate::{
	cpu::{instruction::Register16, Cpu},
	display::Display,
};
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

	pub fn boot(&mut self) {
		self.cpu.registers[Register16::AF] = 0x0100;
		self.cpu.registers[Register16::BC] = 0xFF13;
		self.cpu.registers[Register16::DE] = 0x00C1;
		self.cpu.registers[Register16::HL] = 0x8403;
		self.cpu.registers[Register16::SP] = 0xFFFE;
		self.cpu.pc = 0x100;
	}

	/// fully execute the next instruction
	pub fn step(&mut self) -> u8 {
		self.cpu.step()
	}

	pub fn redraw(&mut self) -> bool {
		let redraw = self.cpu.memory.ppu.redraw;

		if redraw {
			self.cpu.memory.ppu.redraw = !self.cpu.memory.ppu.redraw;
		}

		redraw
	}
}
