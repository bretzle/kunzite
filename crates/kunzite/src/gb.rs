//!

use std::path::Path;

use crate::cpu::{instruction::Register16, Cpu};
use color_eyre::Result;

/// Brings all the components into a single package
pub struct Gb {
	/// the cpu
	pub cpu: Cpu,
}

impl Gb {
	/// Create a new Gameboy instance
	pub fn create() -> Self {
		Self {
			cpu: Cpu::default(),
		}
	}

	/// Insert a rom into the gameboy
	pub fn insert_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
		self.cpu.memory.cartridge.insert_rom(path)
	}

	pub fn boot(&mut self) {
		self.cpu.registers[Register16::AF] = 0x0100;
		self.cpu.registers[Register16::BC] = 0xFF13;
		self.cpu.registers[Register16::DE] = 0x00C1;
		self.cpu.registers[Register16::HL] = 0x8403;
		self.cpu.registers[Register16::SP] = 0xFFFE;
		self.cpu.pc = 0x100;

		self.cpu.memory.write(0xFF00, 0xCF);
		self.cpu.memory.write(0xFF01, 0x00);
		self.cpu.memory.write(0xFF02, 0x7E);
		self.cpu.memory.write(0xFF04, 0x18);
		self.cpu.memory.write(0xFF05, 0x00);
		self.cpu.memory.write(0xFF06, 0x00);
		self.cpu.memory.write(0xFF07, 0xF8);
		self.cpu.memory.write(0xFF0F, 0xE1);
		self.cpu.memory.write(0xFF10, 0x80);
		self.cpu.memory.write(0xFF11, 0xBF);
		self.cpu.memory.write(0xFF12, 0xF3);
		self.cpu.memory.write(0xFF13, 0xFF);
		self.cpu.memory.write(0xFF14, 0xBF);
		self.cpu.memory.write(0xFF16, 0x3F);
		self.cpu.memory.write(0xFF17, 0x00);
		self.cpu.memory.write(0xFF18, 0xFF);
		self.cpu.memory.write(0xFF19, 0xBF);
		self.cpu.memory.write(0xFF1A, 0x7F);
		self.cpu.memory.write(0xFF1B, 0xFF);
		self.cpu.memory.write(0xFF1C, 0x9F);
		self.cpu.memory.write(0xFF1D, 0xFF);
		self.cpu.memory.write(0xFF1E, 0xBF);
		self.cpu.memory.write(0xFF20, 0xFF);
		self.cpu.memory.write(0xFF21, 0x00);
		self.cpu.memory.write(0xFF22, 0x00);
		self.cpu.memory.write(0xFF23, 0xBF);
		self.cpu.memory.write(0xFF24, 0x77);
		self.cpu.memory.write(0xFF25, 0xF3);
		self.cpu.memory.write(0xFF26, 0xF1);
		self.cpu.memory.write(0xFF40, 0x91);
		self.cpu.memory.write(0xFF41, 0x81);
		self.cpu.memory.write(0xFF42, 0x00);
		self.cpu.memory.write(0xFF43, 0x00);
		self.cpu.memory.write(0xFF44, 0x91);
		self.cpu.memory.write(0xFF45, 0x00);
		// self.cpu.memory.write(0xFF46, 0xFF);
		self.cpu.memory.write(0xFF47, 0xFC);
		self.cpu.memory.write(0xFF48, 0xFF);
		self.cpu.memory.write(0xFF49, 0xFF);
		self.cpu.memory.write(0xFF4A, 0x00);
		self.cpu.memory.write(0xFF4B, 0x00);
		self.cpu.memory.write(0xFF4D, 0xFF);
		self.cpu.memory.write(0xFF4F, 0xFF);
		self.cpu.memory.write(0xFF51, 0xFF);
		self.cpu.memory.write(0xFF52, 0xFF);
		self.cpu.memory.write(0xFF53, 0xFF);
		self.cpu.memory.write(0xFF54, 0xFF);
		self.cpu.memory.write(0xFF55, 0xFF);
		self.cpu.memory.write(0xFF56, 0xFF);
		self.cpu.memory.write(0xFF68, 0xFF);
		self.cpu.memory.write(0xFF69, 0xFF);
		self.cpu.memory.write(0xFF6A, 0xFF);
		self.cpu.memory.write(0xFF6B, 0xFF);
		self.cpu.memory.write(0xFF70, 0xFF);
		self.cpu.memory.write(0xFFFF, 0x00);
	}

	/// fully execute the next instruction
	pub fn step(&mut self) -> u8 {
		self.cpu.step()
	}
}
