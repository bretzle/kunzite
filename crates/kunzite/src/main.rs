//!

// #![deny(missing_docs)]
#![feature(exclusive_range_pattern)]
#![feature(const_panic)]

pub mod cpu;
pub mod display;
pub mod emulator;
pub mod gb;
pub mod memory;
mod util;

use color_eyre::Result;
use emulator::Emulator;
use gui::*;

#[allow(clippy::many_single_char_names)]
fn main() -> Result<()> {
	color_eyre::install()?;

	let options = Options::new("GB Emulator", 1000, 600);

	run::<Emulator>(options)

	// const ROM: &[u8] = include_bytes!("../../../roms/cpu_instrs.gb");

	// let mut real_rom = ROM.to_vec();
	// real_rom
	// 	.iter_mut()
	// 	.enumerate()
	// 	.filter(|(idx, _)| (0xA8..0xE0).contains(idx))
	// 	.for_each(|(_, val)| *val = 0);

	// let instuctions = Cpu::try_decode_all(&real_rom);

	// for (pc, inst) in instuctions {
	// 	println!("{:04X} {:?}", pc, inst);
	// }
}
