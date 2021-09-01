mod emulator;

use color_eyre::Result;
use emulator::Emulator;
use gui::*;

fn main() -> Result<()> {
	color_eyre::install()?;

	let options = Options::new("GB Emulator", 600, 400);

	run::<Emulator>(options)
}
