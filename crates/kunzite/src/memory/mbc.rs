mod rom_only;

pub use rom_only::*;

pub trait Mbc {
	fn read_byte(&self, index: u16) -> u8;
	fn write_byte(&mut self, index: u16, val: u8);
	// fn get_cartridge(&self) -> &Cartridge;
	// fn get_cartridge_mut(&mut self) -> &mut Cartridge;
}

#[derive(Debug, Copy, Clone)]
pub enum MbcType {
	RomOnly,
	Mbc1,
	Mbc2,
	Mbc3,
	Mbc5,
	Unknown,
}
