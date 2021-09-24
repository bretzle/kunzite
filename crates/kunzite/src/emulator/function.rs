use super::Emulator;

const FRAME_CYCLES: u32 = 456 * (144 + 10);

pub enum Step {
	InstCount(usize),
	Frame,
}

impl Emulator {
	pub fn update_screen(&mut self) {
		// let fb = self.gb.cpu.memory.ppu.frame_buffer();
		// self.screen_texture.refresh(|x, y| {
		// 	let v = fb[x + (y * 160)];
		// 	[v, v, v]
		// });
	}

	pub fn step(&mut self, step: Step) {
		match step {
			Step::InstCount(count) => {
				for _ in 0..count {
					if self.breakpoints.0
						&& count > 1 && self.breakpoints.1.contains(self.gb.cpu.pc())
					{
						self.run = false;
						break;
					}
					self.ticks += self.gb.step() as u32;
					if self.ticks >= FRAME_CYCLES {
						self.update_screen();
						self.ticks = 0;
					}
				}
			}
			Step::Frame => {
				while self.ticks <= FRAME_CYCLES {
					self.ticks += self.gb.step() as u32;
				}
				self.update_screen();
				self.ticks = 0;
			}
		}
	}
}
