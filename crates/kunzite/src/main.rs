//!

// #![deny(missing_docs)]
#![feature(exclusive_range_pattern)]
#![feature(const_panic)]
#![feature(option_result_unwrap_unchecked)]

pub mod cpu;
pub mod emulator;
pub mod gb;
pub mod memory;
pub mod ppu;
mod util;

use color_eyre::Result;
use emulator::Emulator;
use gui::{run, Options};

#[allow(clippy::many_single_char_names)]
fn main() -> Result<()> {
	color_eyre::install()?;

	let options = Options::new("GB Emulator", 1000, 600);

	run::<Emulator>(options)
}
