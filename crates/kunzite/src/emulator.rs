//!

use std::time::Duration;

use color_eyre::Report;
use gui::{prelude::*, Application};

use crate::{
	cpu::instruction::{Flag, Register16, Register8},
	gb::Gb,
};

/// The emulator
pub struct Emulator {
	gb: Gb,
	run: bool,
}

impl Emulator {
	fn step(&mut self, num: usize) {
		for _ in 0..num {
			self.gb.step();
		}
	}
}

impl Application for Emulator {
	type Error = Report;

	fn setup() -> Self {
		let mut gb = Gb::new();

		gb.insert_rom("roms/bootloader.gb")
			.expect("Failed to load ROM.");

		Self { gb, run: false }
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
			Event::KeyDown {
				keycode: Some(Keycode::Return),
				repeat: false,
				..
			} => {
				self.step(0x1FFF * 3 + 6);
			}
			_ => (),
		}

		Ok(())
	}

	fn update(&mut self, _frame_time: &Duration, _running: &mut bool) -> Result<(), Self::Error> {
		if self.run {
			self.step(100);
		}
		Ok(())
	}

	fn draw(&mut self, ui: &Ui) {
		Window::new(im_str!("Chip-8 - CPU State"))
			.resizable(false)
			.collapsible(false)
			.size([316., 567.], Condition::Always)
			.position([1., 50.], Condition::Always)
			.build(ui, || {
				let a = self.gb.cpu.registers[Register8::A];
				let f = self.gb.cpu.registers.flags();
				let af = self.gb.cpu.registers[Register16::AF];

				let b = self.gb.cpu.registers[Register8::B];
				let c = self.gb.cpu.registers[Register8::C];
				let bc = self.gb.cpu.registers[Register16::BC];

				let d = self.gb.cpu.registers[Register8::D];
				let e = self.gb.cpu.registers[Register8::E];
				let de = self.gb.cpu.registers[Register16::DE];

				let h = self.gb.cpu.registers[Register8::H];
				let l = self.gb.cpu.registers[Register8::L];
				let hl = self.gb.cpu.registers[Register16::HL];

				ui.text(format!(
					"PC: {:04X}  [{:?}]",
					self.gb.cpu.pc,
					self.gb.cpu.parse_instruction().unwrap().1
				));
				ui.text(format!("AF: {:02X}|{:02X} [{:04X}]", a, f, af));
				ui.text(format!("BC: {:02X}|{:02X} [{:04X}]", b, c, bc));
				ui.text(format!("DE: {:02X}|{:02X} [{:04X}]", d, e, de));
				ui.text(format!("HL: {:02X}|{:02X} [{:04X}]", h, l, hl));
				ui.text(format!("SP: {:04X}", self.gb.cpu.registers[Register16::SP]));

				ui.text(format!("Flags:"));
				ui.text(format!("Zero: {}", self.gb.cpu.registers.flag(Flag::Z)));
				ui.text(format!("Subtract: {}", self.gb.cpu.registers.flag(Flag::N)));
				ui.text(format!(
					"Half-carry: {}",
					self.gb.cpu.registers.flag(Flag::H)
				));
				ui.text(format!("Carry: {}", self.gb.cpu.registers.flag(Flag::C)));
			});
	}
}
