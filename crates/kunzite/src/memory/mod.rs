//! Memory module

mod cartridge;

use self::cartridge::Cartridge;
use crate::ppu::PPU;

/// Memory
pub struct Memory {
	pub cartridge: Cartridge,
	ram: [u8; 0x2000],
	hram: [u8; 0x7F],
	serial_io: [u8; 0x4C],
	pub ppu: PPU,
	/// Interrupt flag
	pub int_flag: u8,
	/// Interrupt enable
	pub int_enable: u8,
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
			int_flag: 0,
			int_enable: 0,
			ppu: PPU::new(),
			hram: [0; 0x7F],
		}
	}

	pub fn get(&self, addr: u16) -> Option<u8> {
		let addr = addr as usize;
		let val = match addr {
			0x0000..0x8000 => self.cartridge.read(addr), // cartrige rom
			0x8000..0xA000 => self.ppu.read(addr),       // vram
			0xA000..0xC000 => self.cartridge.read(addr), // switchable ram bank
			0xC000..0xE000 => self.ram[addr & 0x1FFF],   // internal ram
			0xE000..0xFE00 => self.ram[(addr - 0x2000) & 0x1FFF], // copy of internal ram
			0xFE00..0xFEA0 => self.ppu.read(addr),       // sprite attrib memory
			0xFEA0..0xFF0F => 0,                         // prohibited
			0xFF0F => self.int_flag,                     // Interrupt flag
			0xFF10..0xFF40 => 0,                         // ???
			0xFF40..0xFF4C => self.ppu.read(addr),       // PPU (actually io but only need ppu atm)
			0xFF4C..0xFF80 => 0,                         // ???
			0xFF80..0xFFFF => self.hram[addr & 0x7f],    // HRAM
			0xFFFF => self.int_enable,                   // Interrupt enable
			_ => return None,
		};

		Some(val)
	}

	pub fn read(&self, addr: u16) -> u8 {
		unsafe { self.get(addr).unwrap_unchecked() }
	}

	pub fn write(&mut self, addr: u16, val: u8) {
		let addr = addr as usize;
		match addr {
			0x0000..0x8000 => self.cartridge.write(addr, val), // cartrige rom
			0x8000..0xA000 => self.ppu.write(addr, val),       // vram
			0xA000..0xC000 => self.cartridge.write(addr, val), // switchable ram bank
			0xC000..0xE000 => self.ram[addr & 0x1FFF] = val,   // internal ram
			0xE000..0xFE00 => self.ram[(addr - 0x2000) & 0x1FFF] = val, // copy of internal ram
			0xFE00..0xFEA0 => self.ppu.write(addr, val),       // sprite attrib memory
			0xFEA0..0xFF0F => (),                              // prohibited
			0xFF0F => self.int_flag = val,                     // Interrupt flag
			0xFF10..0xFF40 => (),                              // ???
			0xFF40..0xFF4C => self.ppu.write(addr, val),       // PPU
			0xFF4C..0xFF80 => (),                              // ???
			0xFF80..0xFFFF => self.hram[addr & 0x7f] = val,    // HRAM
			0xFFFF => self.int_enable = val,                   // Interrupt enable
			_ => (),
		};
	}

	pub fn update(&mut self, tick: u8) {
		// self.cartridge.update(tick); does nothing
		self.ppu.update(tick);

		if self.ppu.irq_vblank {
			self.int_flag |= 0x1;
			self.ppu.irq_vblank = false;
		}

		if self.ppu.irq_lcdc {
			self.int_flag |= 0x2;
			self.ppu.irq_lcdc = false;
		}
	}
}
