//!

use std::time::Duration;

use color_eyre::Report;
use gui::{prelude::*, Application};

use crate::gb::Gb;

/// The emulator
pub struct Emulator(Gb);

impl Emulator {
	fn step(&mut self, num: usize) {
		for _ in 0..num {
			self.0.step();
		}
	}
}

impl Application for Emulator {
	type Error = Report;

	fn setup() -> Self {
		let mut gb = Gb::new();

		gb.insert_rom("roms/tetris.gb")
			.expect("Failed to load ROM.");

		Self(gb)
	}

	fn handle_event(&mut self, event: Event, running: &mut bool) -> Result<(), Self::Error> {
		match event {
			Event::Quit { .. } => *running = false,
			Event::KeyDown {
				keycode: Some(Keycode::Space),
				repeat: false,
				..
			} => {
				self.step(1);
			}
			_ => (),
		}

		Ok(())
	}

	fn update(&mut self, _frame_time: &Duration, _running: &mut bool) -> Result<(), Self::Error> {
		Ok(())
	}

	fn draw(&mut self, _ui: &Ui) {}
}
