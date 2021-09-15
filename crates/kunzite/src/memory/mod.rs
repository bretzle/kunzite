//! Memory module

mod cartridge;

use std::ops::{Index, IndexMut};

use self::cartridge::Cartridge;

/// Memory
pub struct Memory {
	pub cartridge: Cartridge,
	ram: [u8; 0x2000],
	serial_io: [u8; 16],
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
			serial_io: [0; 16],
		}
	}

	pub fn get(&self, addr: usize) -> Option<u8> {
		let val = match addr {
			0x0000..=0x7fff => self.cartridge[addr],
			0xc000..=0xdfff => self.ram[addr & 0x1fff],
			0xff00..=0xff0f => self.serial_io[addr & 0xF],
			_ => return None,
		};

		Some(val)
	}
}

impl Index<usize> for Memory {
	type Output = u8;

	fn index(&self, addr: usize) -> &Self::Output {
		match addr {
			0x0000..=0x7fff => &self.cartridge[addr],
			0xc000..=0xdfff => &self.ram[addr & 0x1fff],
			0xff00..=0xff0f => &self.serial_io[addr & 0xF],
			_ => {
				println!(
					"{:#06X} is not a valid memory address/not supported yet",
					addr
				);
				&0
			}
		}
	}
}

impl IndexMut<usize> for Memory {
	fn index_mut(&mut self, addr: usize) -> &mut Self::Output {
		match addr {
			0x0000..=0x7fff => &mut self.cartridge[addr],
			0xc000..=0xdfff => &mut self.ram[addr & 0x1fff],
			0xff00..=0xff0f => &mut self.serial_io[addr & 0xF],
			_ => {
				panic!(
					"{:#06X} is not a valid memory address/not supported yet",
					addr
				);
			}
		}
	}
}
