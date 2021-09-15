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
			rom: [0; 0x7fff].into(),
		}
	}

	/// Insert a cartridge into the cpu
	pub fn insert_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
		let mut file = File::open(path)?;

		// let size = file.read_to_end(&mut self.rom[0..])?;
		let size = file.read(&mut self.rom[0..])?;

		println!("Rom size: {} bytes", size);

		Ok(())
	}
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
