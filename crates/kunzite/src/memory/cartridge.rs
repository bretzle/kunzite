use color_eyre::Result;
use std::{fs::File, io::Read, path::Path};

use crate::util::slice_to_string;

#[derive(Default)]
pub struct Cartridge {
	pub rom: Vec<u8>,
	ram: Vec<u8>,
	mbc_type: u8,
	ram_enable: bool,
	bank_no_upper: u8,
	bank_no_lower: u8,
	num_rom_banks: u8,
	mode: bool,

	header: Option<CartridgeHeader>,
}

#[derive(Debug)]
pub struct CartridgeHeader {
	name: String,
	manuf: String,
	cgb: u8,
	licensee: String,
	sgb: u8,
	mbc_type: u8,
	rom_size: u8,
	ram_size: u8,
	dest_code: u8,
	old_licesee: u8,
	version_number: u8,
	header_checksum: u8,
}

impl Cartridge {
	pub fn new() -> Self {
		Self {
			rom: vec![],
			ram: vec![],
			mbc_type: 0,
			ram_enable: false,
			bank_no_upper: 0,
			bank_no_lower: 0,
			num_rom_banks: 0,
			mode: false,
			header: None,
		}
	}

	/// Insert a cartridge into the cpu
	pub fn insert_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
		let mut file = File::open(path)?;

		let _ = file.read_to_end(&mut self.rom)?;

		// let mut file = File::open(bootloader)?;
		// file.read(&mut self.rom[0..0x100])?;

		// println!("Rom size: {} bytes", size);

		let header = self.header.insert(CartridgeHeader {
			name: slice_to_string(&self.rom[0x134..0x143]),
			manuf: slice_to_string(&self.rom[0x13F..0x142]),
			cgb: self.rom[0x143],
			licensee: slice_to_string(&self.rom[0x144..0x145]),
			sgb: self.rom[0x146],
			mbc_type: self.rom[0x147],
			rom_size: self.rom[0x148],
			ram_size: self.rom[0x149],
			dest_code: self.rom[0x14A],
			old_licesee: self.rom[0x14B],
			version_number: self.rom[0x14C],
			header_checksum: self.rom[0x14D],
		});

		let rom_size: usize = match header.rom_size {
			0 => 32 * 1024,
			n => 32 * 1024 << (n as usize),
		};

		let num_rom_banks = 2 << header.rom_size;

		let ram_size: usize = match header.ram_size {
			0 => 0,
			1 => 2 * 1024,
			2 => 8 * 1024,
			3 => 32 * 1024,
			4 => 128 * 1024,
			5 => 64 * 1024,
			_ => panic!("RAM size invalid"),
		};

		let mbc_name = match header.mbc_type {
			0x00 => "ROM ONLY",
			0x01 => "MBC1",
			0x02 => "MBC1+RAM",
			0x03 => "MBC1+RAM+BATTERY",
			0x05 => "MBC2",
			0x06 => "MBC2+BATTERY",
			0x08 => "ROM+RAM",
			0x09 => "ROM+RAM+BATTERY",
			0x0b => "MMM01",
			0x0c => "MMM01+RAM",
			0x0d => "MMM01+RAM+BATTERY",
			0x0f => "MBC3+TIMER+BATTERY",
			0x10 => "MBC3+TIMER+RAM+BATTERY",
			0x11 => "MBC3",
			0x12 => "MBC3+RAM",
			0x13 => "MBC3+RAM+BATTERY",
			0x19 => "MBC5",
			0x1a => "MBC5+RAM",
			0x1b => "MBC5+RAM+BATTERY",
			0x1c => "MBC5+RUMBLE",
			0x1d => "MBC5+RUMBLE+RAM",
			0x1e => "MBC5+RUMBLE+RAM+BATTERY",
			0x20 => "MBC6",
			0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
			0xfc => "POCKET CAMERA",
			0xfd => "BANDAI TAMA5",
			0xfe => "HuC3",
			0xff => "HuC1+RAM+BATTERY",
			_ => "Unknown",
		};

		let mut chksum: u8 = 0;
		for i in 0x0134..0x014d {
			chksum = chksum.wrapping_sub(self.rom[i]).wrapping_sub(1);
		}

		if rom_size != self.rom.len() {
			panic!("ROM file invalid");
		}

		if chksum != header.header_checksum {
			panic!("ROM header checksum is incorrect");
		}

		self.ram = vec![0; ram_size];
		self.mbc_type = header.mbc_type;
		self.ram_enable = false;
		self.bank_no_upper = 0;
		self.bank_no_lower = 0;
		self.num_rom_banks = num_rom_banks;
		self.mode = false;

		println!("ROM size {}KB", rom_size / 1024);
		println!("RAM size {}KB", ram_size / 1024);
		println!("MBC type {}", mbc_name);

		println!("{:#?}", header);

		Ok(())
	}

	fn rom_bank_no(&self) -> u8 {
		let bank_no = if self.mode {
			self.bank_no_lower
		} else {
			self.bank_no_upper << 5 | self.bank_no_lower
		};

		let bank_no = match bank_no {
			0 | 0x20 | 0x40 | 0x60 => bank_no + 1,
			_ => bank_no,
		};

		bank_no & (self.num_rom_banks - 1)
	}

	fn ram_bank_no(&self) -> u8 {
		if self.mode {
			self.bank_no_upper
		} else {
			0
		}
	}
}

impl Cartridge {
	pub fn read(&self, addr: usize) -> u8 {
		match addr {
			// ROM bank 00
			0x0000..0x4000 => self.rom[addr as usize],
			// ROM bank 01-7f
			0x4000..0x8000 => {
				let offset = (16 * 1024) * self.rom_bank_no() as usize;
				self.rom[(addr & 0x3fff) as usize + offset]
			}
			// RAM bank 00-03
			0xA000..0xC000 => {
				if !self.ram_enable {
					return 0xff;
				}
				let offset = (8 * 1024) * self.ram_bank_no() as usize;
				self.ram[(addr & 0x1fff) as usize + offset]
			}
			_ => unreachable!("Unexpected address: 0x{:04x}", addr),
		}
	}

	pub fn write(&mut self, addr: usize, val: u8) {
		match addr {
			// RAM enable
			0x0000..0x2000 => self.ram_enable = val & 0x0F == 0x0A,
			// ROM bank number (lower 5 bits)
			0x2000..0x4000 => self.bank_no_lower = val & 0x1F,
			// RAM bank number or ROM bank number (upper 2 bits)
			0x4000..0x6000 => self.bank_no_upper = val & 0x03,
			// ROM/RAM mode select
			0x6000..0x8000 => self.mode = val & 0x01 > 0,
			// RAM bank 00-03
			0xA000..0xC000 => {
				if !self.ram_enable {
					return;
				}
				let offset = (8 * 1024) * self.ram_bank_no() as usize;
				self.ram[(addr & 0x1FFF) as usize + offset] = val
			}
			_ => unreachable!("Unexpected address: 0x{:04x}", addr),
		}
	}
}
