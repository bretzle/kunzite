//!

use std::{collections::HashSet, time::Duration};

use color_eyre::Report;
use gui::{prelude::*, Application};

use crate::{
	cpu::instruction::{Flag, Instruction, Register16, Register8},
	gb::Gb,
	memory::Memory,
};

/// The emulator
pub struct Emulator {
	gb: Gb,
	run: bool,
	locs: HashSet<Instruction>,
}

impl Emulator {
	fn step(&mut self, num: usize) {
		for _ in 0..num {
			// if self.gb.cpu.pc == 0xC000 {
			// 	continue;
			// }
			if let Some(i) = self.gb.cpu.parse_instruction() {
				if self.run {
					match i {
						Instruction::Jp(_, 0xC000) | Instruction::Rst(_) => {
							println!("here");
							self.run = false;
							return;
						}
						_ => (),
					}
				}
				self.locs.insert(i);
			}

			self.gb.step();

			let s = self.gb.cpu.memory[0xFF02];

			if s != 0 {
				dbg!(s);
			}

			if s == 0x81 {
				println!("{}", self.gb.cpu.memory[0xFF01]);
				self.gb.cpu.memory[0xFF02] = 0;
			}
		}
	}

	fn skip(&mut self, count: usize) {
		for _ in 0..count {
			let inst = self.gb.cpu.parse_instruction().unwrap();
			self.gb.cpu.pc += inst.size();
		}
	}
}

impl Application for Emulator {
	type Error = Report;

	fn setup() -> Self {
		let mut gb = Gb::new();

		gb.boot();

		// gb.insert_rom("roms/bootloader.gb")
		gb.insert_rom("roms/cpu_instrs.gb")
			.expect("Failed to load ROM.");

		Self {
			gb,
			run: false,
			locs: HashSet::new(),
		}
	}

	fn handle_event(&mut self, event: Event, running: &mut bool) -> Result<(), Self::Error> {
		match event {
			Event::Quit { .. } => *running = false,
			Event::KeyDown {
				keycode: Some(Keycode::Space),
				// repeat: false,
				..
			} => {
				self.step(1);
			}
			Event::KeyDown {
				keycode: Some(Keycode::Return),
				repeat: false,
				..
			} => {
				self.run = !self.run;
			}
			Event::KeyDown {
				keycode: Some(Keycode::Semicolon),
				repeat: false,
				..
			} => {
				self.skip(1);
			}
			Event::KeyDown {
				keycode: Some(Keycode::P),
				..
			} => {
				println!("{:#?}", self.locs)
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
		Window::new(im_str!("CPU State")).build(ui, || {
			let a = self.gb.cpu.read(Register8::A);
			let f = self.gb.cpu.registers.flags();
			let af = self.gb.cpu.registers[Register16::AF];

			let b = self.gb.cpu.read(Register8::B);
			let c = self.gb.cpu.read(Register8::C);
			let bc = self.gb.cpu.registers[Register16::BC];

			let d = self.gb.cpu.read(Register8::D);
			let e = self.gb.cpu.read(Register8::E);
			let de = self.gb.cpu.registers[Register16::DE];

			let h = self.gb.cpu.read(Register8::H);
			let l = self.gb.cpu.read(Register8::L);
			let hl = self.gb.cpu.registers[Register16::HL];

			let i_text = match self.gb.cpu.parse_instruction() {
				Some(i) => {
					format!("PC: {:04X}  [{:?}]", self.gb.cpu.pc, i)
				}
				None => format!("PC: {:04X}  [END]", self.gb.cpu.pc),
			};

			ui.text(i_text);
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

		// Window::new(im_str!("memory")).build(ui, || {
		// 	//
		// 	let addr = self.gb.cpu.last_mem_addr;

		// 	let range = if addr >= 3 {
		// 		(addr - 3)..(addr + 3)
		// 	} else {
		// 		0..(addr + 7)
		// 	};

		// 	for idx in range {
		// 		ui.text(format!("0x{:04X}: {:02X}", idx, self.gb.memory[idx]));
		// 	}
		// });
		Window::new(im_str!("Memory")).build(ui, || {
			ui.set_next_item_width(-1.);

			Slider::new(im_str!("##")).range(1..=16).build(ui, &mut 16);

			let memory = &self.gb.cpu.memory;

			ChildWindow::new("memory").build(ui, || {
				let total_addresses = Memory::LENGTH;
				let lines_to_draw = total_addresses / 16 as usize;
				let last_line_address = (lines_to_draw - 1) * 16 as usize;
				let mut last_line_items = total_addresses % 16 as usize;

				if last_line_items == 0 {
					last_line_items = 16 as usize
				}

				let clipper = ListClipper::new(lines_to_draw as i32);
				let mut ctoken = clipper.begin(ui);

				while ctoken.step() {
					for offset in ctoken.display_start()..ctoken.display_end() {
						let address = offset as usize * 16 as usize;

						match address {
							0x100 | 0x8000 | 0xA000 | 0xC000 | 0xE000 | 0xFE00 | 0xFEA0
							| 0xFF00 | 0xFF4C | 0xFF80 | 0xFFFF => ui.separator(),
							_ => {}
						}

						let max_items = 16 as usize;

						let mut item_count = max_items;
						if address == last_line_address {
							item_count = last_line_items;
						}

						// display address
						ui.text(format!("{:#05X} |", address));

						// display address content (hex)
						for base in 0..item_count {
							ui.same_line();
							ui.text(format!("{:>02X}", memory[address + base]))
						}

						for _ in item_count..max_items {
							ui.same_line();
							ui.text("..");
						}

						// display address content (ascii)
						ui.same_line();
						let mut text = "| ".to_string();

						for base in 0..item_count {
							let byte = memory[address + base] as char;
							let c = if byte.is_ascii_control() || byte.is_ascii_whitespace() {
								' '
							} else {
								byte
							};
							text.push(c);
						}
						for _ in item_count..max_items {
							text.push(' ');
						}

						ui.text(text)
					}
				}
			});
		});
	}
}
