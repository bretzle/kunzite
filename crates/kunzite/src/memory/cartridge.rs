use color_eyre::Result;
use std::{
	fs::File,
	io::Read,
	ops::{Index, IndexMut},
	path::Path,
};

#[derive(Default)]
pub struct Cartridge {
	pub rom: Vec<u8>,
}

impl Cartridge {
	pub fn new() -> Self {
		Self {
			rom: [0; 0x8000].into(),
		}
	}

	/// Insert a cartridge into the cpu
	pub fn insert_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
		let mut file = File::open(path)?;

		// let size = file.read_to_end(&mut self.rom[0..])?;
		let size = file.read(&mut self.rom[0..])?;

		let name = slice_to_string(&self.rom[0x134..0x143]);
		let manuf = slice_to_string(&self.rom[0x13F..0x142]);
		let cgb = self.rom[0x143];
		let licensee = slice_to_string(&self.rom[0x144..0x145]);
		let sgb = self.rom[0x146];
		let cartridge_type = self.rom[0x147];
		let rom_size = self.rom[0x148];
		let ram_size = self.rom[0x149];
		let dest_code = self.rom[0x14A];
		let old_licesee = self.rom[0x14B];
		let version_number = self.rom[0x14C];
		let header_checksum = self.rom[0x14D];

		println!("Rom size: {} bytes", size);
		println!("Rom name: {}", name);
		println!("Manufacture: {}", manuf);
		println!("cgb: {}", cgb);
		println!("licensee: {}", licensee);
		println!("sgb: {}", sgb);
		println!("cartridge_type: {}", cartridge_type);
		println!("rom_size: {}", rom_size);
		println!("ram_size: {}", ram_size);
		println!("dest_code: {}", dest_code);
		println!("old_licesee{}", old_licesee);
		println!("version_number: {}", version_number);
		println!("header_checksum: {}", header_checksum);

		Ok(())
	}
}

fn slice_to_string(slice: &[u8]) -> String {
	slice.iter().map(|b| *b as char).collect()
}

impl Index<usize> for Cartridge {
	type Output = u8;

	fn index(&self, index: usize) -> &Self::Output {
		&self.rom[index]
	}
}

impl IndexMut<usize> for Cartridge {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.rom[index]
	}
}
