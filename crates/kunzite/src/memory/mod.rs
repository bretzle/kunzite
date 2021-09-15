//! Memory module

mod cartridge;

use std::ops::{Index, IndexMut};

use self::cartridge::Cartridge;

/// Memory
pub struct Memory {
	pub cartridge: Cartridge,
	ram: [u8; 0x2000],
	serial_io: [u8; 0x4C],
	/// Interrupt enable
	pub int_enable: u8,
	vram: [u8; 0x2000],
	boot_ram: [u8; 0x7F],
}

impl Default for Memory {
	fn default() -> Self {
		Self::new()
	}
}

impl Memory {
	pub const LENGTH: usize = 0x10000;

	/// Create a new memory instance
	pub fn new() -> Self {
		Self {
			cartridge: Cartridge::new(),
			ram: [0; 0x2000],
			serial_io: [0; 0x4C],
			int_enable: 0,
			vram: [0; 0x2000],
			boot_ram: [0; 0x7F],
		}
	}

	pub fn get(&self, addr: usize) -> Option<u8> {
		let val = match addr {
			0x0000..=0x7fff => self.cartridge[addr],
			0xc000..=0xdfff => self.ram[addr & 0x1fff],
			0xff00..=0xff4C => self.serial_io[addr & 0x7F],
			_ => return None,
		};

		Some(val)
	}
}

impl Index<usize> for Memory {
	type Output = u8;

	fn index(&self, addr: usize) -> &Self::Output {
		match addr {
			0x0000..0x8000 => &self.cartridge[addr],     // cartrige rom
			0x8000..0xA000 => &self.vram[addr - 0x8000], // vram
			0xA000..0xC000 => &0,                        // switchable ram bank
			0xC000..0xE000 => &self.ram[addr & 0x1fff],  // internal ram
			0xE000..0xFE00 => &0,                        // copy of internal ram
			0xFE00..0xFEA0 => &0,                        // sprite attrib memory
			0xFEA0..0xFF00 => &0,                        // empty but usable for io
			0xFF00..0xFF4C => &self.serial_io[addr & 0x7F], // io ports
			0xFF4C..0xFF80 => &0,                        // empty but usable for io
			0xFF80..0xFFFF => &self.boot_ram[addr - 0xFF80], // internal ram,
			0xFFFF => &self.int_enable,                  // interupt enable register,
			_ => unreachable!(),
		}
	}
}

impl IndexMut<usize> for Memory {
	fn index_mut(&mut self, addr: usize) -> &mut Self::Output {
		match addr {
			0x0000..0x8000 => &mut self.cartridge[addr], // cartrige rom
			0x8000..0xA000 => &mut self.vram[addr - 0x8000], // vram
			// 0xA000..0xC000 => &mut 0,                        // switchable ram bank
			0xC000..0xE000 => &mut self.ram[addr & 0x1fff], // internal ram
			// 0xE000..0xFE00 => &mut 0,                        // copy of internal ram
			// 0xFE00..0xFEA0 => &mut 0,                        // sprite attrib memory
			// 0xFEA0..0xFF00 => &mut 0,                        // empty but usable for io
			0xFF00..0xFF4C => &mut self.serial_io[addr & 0x7F], // io ports
			// 0xFF4C..0xFF80 => &mut 0,                        // empty but usable for io
			0xFF80..0xFFFF => &mut self.boot_ram[addr - 0xFF80], // internal ram,
			0xFFFF => &mut self.int_enable,                      // interupt enable register,
			_ => {
				panic!(
					"{:#06X} is not a valid memory address/not supported yet",
					addr
				);
			}
		}
	}
}
