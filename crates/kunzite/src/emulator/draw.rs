use super::Emulator;
use crate::{
	cpu::instruction::{Flag, Register16, Register8},
	memory::Memory,
};
use gui::prelude::*;

const SCREEN_WIDTH: f32 = 160.0;
const SCREEN_HEIGHT: f32 = 144.0;
const ZOOM_FACTOR: f32 = 2.0;

impl Emulator {
	pub fn render_cpu_state(&self, ui: &Ui) {
		Window::new("CPU State").build(ui, || {
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

			ui.text("Flags:");
			ui.text(format!("Zero: {}", self.gb.cpu.registers.flag(Flag::Z)));
			ui.text(format!("Subtract: {}", self.gb.cpu.registers.flag(Flag::N)));
			ui.text(format!(
				"Half-carry: {}",
				self.gb.cpu.registers.flag(Flag::H)
			));
			ui.text(format!("Carry: {}", self.gb.cpu.registers.flag(Flag::C)));
			ui.text(format!("Ticks: {}", self.gb.cpu.tick));
		});
	}

	pub fn draw_memory(&self, ui: &Ui) {
		Window::new("Memory").build(ui, || {
			ui.set_next_item_width(-1.);

			Slider::new("##", 1, 16).build(ui, &mut 16);

			let memory = &self.gb.cpu.memory;

			ChildWindow::new("memory").build(ui, || {
				const TOTAL_ADDRESSES: usize = Memory::LENGTH;
				const LINES_TO_DRAW: usize = TOTAL_ADDRESSES / 16;
				const LAST_LINE_ADDRESS: u16 = ((LINES_TO_DRAW - 1) * 16) as u16;
				let mut last_line_items = TOTAL_ADDRESSES as u16 % 16;

				if last_line_items == 0 {
					last_line_items = 16
				}

				let clipper = ListClipper::new(LINES_TO_DRAW as i32);
				let mut ctoken = clipper.begin(ui);

				while ctoken.step() {
					for offset in ctoken.display_start()..ctoken.display_end() {
						let address = offset as u16 * 16;

						match address {
							0x100 | 0x8000 | 0xA000 | 0xC000 | 0xE000 | 0xFE00 | 0xFEA0
							| 0xFF00 | 0xFF4C | 0xFF80 | 0xFFFF => ui.separator(),
							_ => {}
						}

						let max_items = 16;

						let mut item_count = max_items;
						if address == LAST_LINE_ADDRESS {
							item_count = last_line_items;
						}

						// display address
						ui.text(format!("{:#05X} |", address));

						// display address content (hex)
						for base in 0..item_count {
							ui.same_line();
							ui.text(format!("{:>02X}", memory.read(address + base)))
						}

						for _ in item_count..max_items {
							ui.same_line();
							ui.text("..");
						}

						// display address content (ascii)
						ui.same_line();
						let mut text = "| ".to_string();

						for base in 0..item_count {
							let byte = memory.read(address + base) as char;
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

	pub fn draw_display(&self, ui: &Ui) {
		Window::new("Display").resizable(false).build(ui, || {
			Image::new(self.screen_texture.texture_id, [
				SCREEN_WIDTH * ZOOM_FACTOR,
				SCREEN_HEIGHT * ZOOM_FACTOR,
			])
			.build(ui);
		});
	}
}
