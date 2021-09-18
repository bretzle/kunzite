/// Width of screen in pixels.
const SCREEN_W: u8 = 160;
/// Height of screen in pixels.
const SCREEN_H: u8 = 144;

#[derive(Copy, Clone, PartialEq)]
enum BGPriority {
	Color0,
	Color123,
}

/// Pixel Processing Unit.
pub struct PPU {
	/// VRAM
	vram: [u8; 0x2000],
	/// OAM
	oam: [u8; 0xa0],
	/// LCD Control
	lcdc: u8,
	/// Status
	stat: u8,
	/// Scroll Y
	scy: u8,
	/// Scroll X
	scx: u8,
	/// Y-Coordinate
	ly: u8,
	/// LY Compare
	lyc: u8,
	/// DMA Transfer and Start Address
	dma: u8,
	/// Background Palette Data
	bgp: u8,
	/// Object Palette 0 Data
	obp0: u8,
	/// Object Palette 1 Data
	obp1: u8,
	/// Window Y Position
	wy: u8,
	/// Window X Position minus 7
	wx: u8,
	/// V-Blank interrupt request
	pub irq_vblank: bool,
	/// LCDC interrupt request
	pub irq_lcdc: bool,
	/// Elapsed clocks in current mode
	counter: u16,
	/// Frame buffer
	frame_buffer: [u8; (SCREEN_W as usize) * (SCREEN_H as usize)],
	/// Current scanline
	scanline: [u8; SCREEN_W as usize],
	/// Background priority
	bg_prio: [BGPriority; SCREEN_W as usize],
}

impl PPU {
	pub fn new() -> Self {
		Self {
			vram: [0; 0x2000],
			oam: [0; 0xa0],
			lcdc: 0x80,
			stat: 0x02,
			scy: 0,
			scx: 0,
			ly: 0,
			lyc: 0,
			dma: 0,
			bgp: 0,
			obp0: 0,
			obp1: 0,
			wy: 0,
			wx: 0,
			irq_vblank: false,
			irq_lcdc: false,
			counter: 0,
			scanline: [0; SCREEN_W as usize],
			frame_buffer: [0; (SCREEN_W as usize) * (SCREEN_H as usize)],
			bg_prio: [BGPriority::Color0; SCREEN_W as usize],
		}
	}

	/// Checks LCD mode interrupt.
	fn update_mode_interrupt(&mut self) {
		// Mode interrupts
		match self.stat & 0x3 {
			// H-Blank interrupt
			0 if self.stat & 0x8 > 0 => self.irq_lcdc = true,
			// V-Blank interrupt
			1 if self.stat & 0x10 > 0 => self.irq_lcdc = true,
			// OAM Search interrupt
			2 if self.stat & 0x20 > 0 => self.irq_lcdc = true,
			_ => (),
		}
	}

	/// Checks LYC interrupt.
	fn update_lyc_interrupt(&mut self) {
		// LYC=LY coincidence interrupt
		if self.ly == self.lyc {
			self.stat |= 0x4;

			if self.stat & 0x40 > 0 {
				self.irq_lcdc = true;
			}
		} else {
			self.stat &= !0x4;
		}
	}
}

impl PPU {
	pub fn write(&mut self, addr: usize, val: u8) {
		match addr {
			// VRAM
			0x8000..=0x9FFF => {
				// VRAM is inaccessible during pixel transfer
				if self.stat & 0x3 != 3 {
					self.vram[(addr & 0x1fff) as usize] = val
				}
			}

			// OAM
			0xFE00..=0xFE9f => {
				// OAM is only accessible during H-Blank and V-Blank
				if self.stat & 0x3 == 0 || self.stat & 0x3 == 1 {
					self.oam[(addr & 0x00ff) as usize] = val;
				}
			}

			// IO registers
			0xFF40 => {
				if self.lcdc & 0x80 != val & 0x80 {
					self.ly = 0;
					self.counter = 0;

					let mode = if val & 0x80 > 0 { 2 } else { 0 };
					self.stat = (self.stat & 0xf8) | mode;
					self.update_mode_interrupt();
				}

				self.lcdc = val;
			}
			0xFF41 => self.stat = (val & 0xf8) | (self.stat & 0x3),
			0xFF42 => self.scy = val,
			0xFF43 => self.scx = val,
			0xFF44 => (),
			0xFF45 => {
				if self.lyc != val {
					self.lyc = val;
					self.update_lyc_interrupt();
				}
			}
			0xFF47 => self.bgp = val,
			0xFF48 => self.obp0 = val,
			0xFF49 => self.obp1 = val,
			0xFF4a => self.wy = val,
			0xFF4b => self.wx = val,

			_ => unreachable!("Unexpected address: 0x{:04X}", addr),
		}
	}

	pub fn read(&self, addr: usize) -> u8 {
		match addr {
			// VRAM
			0x8000..0xA000 => {
				// VRAM is inaccessible during pixel transfer
				if self.stat & 0x3 != 3 {
					self.vram[(addr & 0x1FFF) as usize]
				} else {
					0xFF
				}
			}

			// OAM
			0xFE00..0xFEA0 => {
				// OAM is only accessible during H-Blank and V-Blank
				if self.stat & 0x3 == 0 || self.stat & 0x3 == 1 {
					self.oam[(addr & 0x00FF) as usize]
				} else {
					0xFF
				}
			}

			// IO registers
			0xFF40 => self.lcdc,
			0xFF41 => self.stat,
			0xFF42 => self.scy,
			0xFF43 => self.scx,
			0xFF44 => self.ly,
			0xFF45 => self.lyc,
			0xFF46 => self.dma,
			0xFF47 => self.bgp,
			0xFF48 => self.obp0,
			0xFF49 => self.obp1,
			0xFF4A => self.wy,
			0xFF4B => self.wx,

			_ => unreachable!("Unexpected address: 0x{:04X}", addr),
		}
	}
}
