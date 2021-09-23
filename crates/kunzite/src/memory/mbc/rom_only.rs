use crate::memory::cartridge::Cartridge;

use super::Mbc;

pub struct RomOnly {
	cartridge: Cartridge,
}

impl RomOnly {
	pub fn new(cartridge: Cartridge) -> Self {
		Self { cartridge }
	}
}

impl Mbc for RomOnly {
	fn read_byte(&self, index: u16) -> u8 {
		match index {
			0x0000..=0x7FFF => {
				let rom = self.cartridge.get_rom();
				rom[index as usize]
			}
			0xA000..=0xBFFF => {
				if self.cartridge.get_ram_size() > 0 {
					let ram = self.cartridge.get_ram();
					ram[index as usize - 0xA000]
				} else {
					0xFF
				}
			}
			_ => unreachable!("Unsupported address: {:04X}", index),
		}
	}

	fn write_byte(&self, index: u16, val: u8) {
		match index {
			0x0000..=0x7FFF => (), // dont allow writing to the rom
			0xA000..=0xBFFF => {
				if self.cartridge.get_ram_size() > 0 {
					let ram = self.cartridge.get_ram_mut();
					ram[index as usize - 0xA000] = val;
				}
			}
			_ => panic!("index out of range: {:04X}", index),
		}
	}
}
