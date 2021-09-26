use crate::memory::Cartridge;

use super::Mbc;

pub struct Mbc1 {
	cartridge: Cartridge,
	selected_rom_bank: usize,
	selected_eram_bank: usize,
	in_ram_banking_mode: bool,
	external_ram_enabled: bool,
	higher_rom_bank_bits: usize,
}

impl Mbc for Mbc1 {
	fn read_byte(&self, index: u16) -> u8 {
		match index {
			0x0000..=0x3FFF => {
				let rom = self.cartridge.get_rom();
				rom[index as usize]
			}
			0x4000..=0x7FFF => {
				let rom = self.cartridge.get_rom();
				let offset = self.selected_rom_bank * 0x4000;
				rom[index as usize - 0x4000 + offset]
			}
			0xA000..=0xBFFF => {
				if self.external_ram_enabled {
					let selected_bank = if self.in_ram_banking_mode {
						self.selected_eram_bank
					} else {
						0
					};
					let offset = selected_bank * 0x2000;
					let address = index as usize - 0xA000 + offset;
					let ram = self.cartridge.get_ram();
					ram[address]
				} else {
					0xFF
				}
			}
			_ => panic!("index out of range: {:04X}", index),
		}
	}

	fn write_byte(&mut self, index: u16, val: u8) {
		match index {
			0x0000..=0x1FFF => {
				if self.cartridge.get_ram_size() > 0 {
					self.external_ram_enabled = (val & 0x0F) == 0x0A
				}
			}
			0x2000..=0x3FFF => {
				if self.in_ram_banking_mode {
					self.selected_rom_bank = usize::from(val & 0x1F);
				} else {
					self.selected_rom_bank =
						usize::from(val & 0x1F) | (self.higher_rom_bank_bits << 5);
				}

				if self.selected_rom_bank == 0x00
					|| self.selected_rom_bank == 0x20
					|| self.selected_rom_bank == 0x40
					|| self.selected_rom_bank == 0x60
				{
					self.selected_rom_bank += 1;
				}

				self.selected_rom_bank &= self.cartridge.get_rom_banks() - 1;
			}
			0x4000..=0x5FFF => {
				if self.in_ram_banking_mode {
					self.selected_eram_bank = usize::from(val & 0x03);
					self.selected_eram_bank &= self.cartridge.get_ram_banks() - 1;
				} else {
					self.higher_rom_bank_bits = usize::from(val & 0x03);
					self.selected_rom_bank =
						(self.selected_rom_bank & 0x1F) | (self.higher_rom_bank_bits << 5);

					if self.selected_rom_bank == 0x00
						|| self.selected_rom_bank == 0x20
						|| self.selected_rom_bank == 0x40
						|| self.selected_rom_bank == 0x60
					{
						self.selected_rom_bank += 1;
					}
					self.selected_rom_bank &= self.cartridge.get_rom_banks() - 1;
				}
			}
			0x6000..=0x7FFF => {
				if !(self.cartridge.get_ram_size() != 3 && (val & 0x01 != 0)) {
					self.in_ram_banking_mode = (val & 1) == 1;
				}
			}
			0xA000..=0xBFFF => {
				if self.external_ram_enabled {
					let selected_bank = if self.in_ram_banking_mode {
						self.selected_eram_bank
					} else {
						0
					};
					let offset = selected_bank * 0x2000;
					let address = index as usize - 0xA000 + offset;
					let ram = self.cartridge.get_ram_mut();
					ram[address] = val;
				}
			}
			_ => panic!("index out of range: {:04X}", index),
		}
	}
}

impl Mbc1 {
	pub fn new(cartridge: Cartridge) -> Self {
		Self {
			cartridge,
			selected_rom_bank: 1,
			selected_eram_bank: 0,
			in_ram_banking_mode: false,
			external_ram_enabled: false,
			higher_rom_bank_bits: 0,
		}
	}
}
