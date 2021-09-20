//!

mod draw;
mod function;

use self::function::Step;
use crate::gb::Gb;
use color_eyre::Report;
use gui::prelude::*;
use std::time::Duration;

/// The emulator
pub struct Emulator {
	gb: Gb,
	run: bool,
	screen_texture: DrawTexture,
	ticks: u32,
	breakpoints: (bool, Vec<u16>),
}

impl Application for Emulator {
	type Error = Report;

	fn setup(system: &mut System) -> Self {
		let mut gb = Gb::create();

		let screen_texture = system.create_texture(160, 144);

		// gb.boot();

		gb.insert_rom("roms/bootloader.gb", "roms/dmg-acid2.gb")
			.expect("Failed to load ROM.");

		Self {
			gb,
			run: false,
			screen_texture,
			ticks: 0,
			breakpoints: (false, vec![0x8e]),
		}
	}

	fn handle_event(&mut self, event: Event, running: &mut bool) -> Result<(), Self::Error> {
		match event {
			Event::Quit => *running = false,
			Event::DroppedFile(_) => {}
			Event::Keypress {
				keycode: Some(key),
				repeat: false,
				..
			} => match key {
				VirtualKeyCode::Space => self.step(Step::InstCount(1)),
				VirtualKeyCode::F => self.step(Step::Frame),
				VirtualKeyCode::Return => self.run = !self.run,
				VirtualKeyCode::D => self.breakpoints.0 = !self.breakpoints.0,
				// VirtualKeyCode::P => {
				// 	println!("{}", self.gb.cpu.memory.ppu.temp.iter().max().unwrap());
				// }
				// VirtualKeyCode::A => {
				// 	while self.gb.cpu.memory.ppu.ly != 144 {
				// 		self.step(Step::InstCount(1));
				// 	}
				// }
				_ => {}
			},
			_ => {}
		}

		Ok(())
	}

	fn update(&mut self, _frame_time: &Duration, _running: &mut bool) -> Result<(), Self::Error> {
		if self.run {
			self.step(Step::InstCount(1000));
		}

		Ok(())
	}

	fn draw(&mut self, ui: &Ui) {
		self.render_cpu_state(ui);
		self.draw_memory(ui);
		self.draw_display(ui);
	}
}
