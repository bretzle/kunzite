//!

use std::time::Duration;

use color_eyre::Report;
// use gui::{prelude::*, Application};

/// The emulator
pub struct Emulator {}

// impl Application for Emulator {
// 	type Error = Report;

// 	fn setup() -> Self {
// 		Self {}
// 	}

// 	fn handle_event(&mut self, event: Event, running: &mut bool) -> Result<(), Self::Error> {
// 		if let Event::Quit { .. } = event {
// 			*running = false
// 		}

// 		Ok(())
// 	}

// 	fn update(&mut self, _frame_time: &Duration, _running: &mut bool) -> Result<(), Self::Error> {
// 		Ok(())
// 	}

// 	fn draw(&mut self, _ui: &Ui) {}
// }
