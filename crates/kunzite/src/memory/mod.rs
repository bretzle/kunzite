mod cartridge;
mod interrupt;
mod mbc;

use self::mbc::{Mbc, MbcType, RomOnly};
use crate::{
	audio::Audio,
	memory::mbc::Mbc1,
	util::{is_set, set_bit, unset_bit, Color},
};

pub use cartridge::Cartridge;

const INITIAL_VALUES_FOR_FFXX: [u8; 0x100] = [
	0xCF, 0x00, 0x7E, 0xFF, 0xD3, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xE1,
	0x80, 0xBF, 0xF3, 0xFF, 0xBF, 0xFF, 0x3F, 0x00, 0xFF, 0xBF, 0x7F, 0xFF, 0x9F, 0xFF, 0xBF, 0xFF,
	0xFF, 0x00, 0x00, 0xBF, 0x77, 0xF3, 0xF1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0x71, 0x72, 0xD5, 0x91, 0x58, 0xBB, 0x2A, 0xFA, 0xCF, 0x3C, 0x54, 0x75, 0x48, 0xCF, 0x8F, 0xD9,
	0x91, 0x80, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFC, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0x2B, 0x0B, 0x64, 0x2F, 0xAF, 0x15, 0x60, 0x6D, 0x61, 0x4E, 0xAC, 0x45, 0x0F, 0xDA, 0x92, 0xF3,
	0x83, 0x38, 0xE4, 0x4E, 0xA7, 0x6C, 0x38, 0x58, 0xBE, 0xEA, 0xE5, 0x81, 0xB4, 0xCB, 0xBF, 0x7B,
	0x59, 0xAD, 0x50, 0x13, 0x5E, 0xF6, 0xB3, 0xC1, 0xDC, 0xDF, 0x9E, 0x68, 0xD7, 0x59, 0x26, 0xF3,
	0x62, 0x54, 0xF8, 0x36, 0xB7, 0x78, 0x6A, 0x22, 0xA7, 0xDD, 0x88, 0x15, 0xCA, 0x96, 0x39, 0xD3,
	0xE6, 0x55, 0x6E, 0xEA, 0x90, 0x76, 0xB8, 0xFF, 0x50, 0xCD, 0xB5, 0x1B, 0x1F, 0xA5, 0x4D, 0x2E,
	0xB4, 0x09, 0x47, 0x8A, 0xC4, 0x5A, 0x8C, 0x4E, 0xE7, 0x29, 0x50, 0x88, 0xA8, 0x66, 0x85, 0x4B,
	0xAA, 0x38, 0xE7, 0x6B, 0x45, 0x3E, 0x30, 0x37, 0xBA, 0xC5, 0x31, 0xF2, 0x71, 0xB4, 0xCF, 0x29,
	0xBC, 0x7F, 0x7E, 0xD0, 0xC7, 0xC3, 0xBD, 0xCF, 0x59, 0xEA, 0x39, 0x01, 0x2E, 0x00, 0x69, 0x00,
];

const INITIAL_VALUES_FOR_COLOR_FFXX: [u8; 0x100] = [
	0xCF, 0x00, 0x7C, 0xFF, 0x44, 0x00, 0x00, 0xF8, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xE1,
	0x80, 0xBF, 0xF3, 0xFF, 0xBF, 0xFF, 0x3F, 0x00, 0xFF, 0xBF, 0x7F, 0xFF, 0x9F, 0xFF, 0xBF, 0xFF,
	0xFF, 0x00, 0x00, 0xBF, 0x77, 0xF3, 0xF1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
	0x91, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFC, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x7E, 0xFF, 0xFE,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x3E, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xC0, 0xFF, 0xC1, 0x00, 0xFE, 0xFF, 0xFF, 0xFF,
	0xF8, 0xFF, 0x00, 0x00, 0x00, 0x8F, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
	0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
	0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
	0x45, 0xEC, 0x42, 0xFA, 0x08, 0xB7, 0x07, 0x5D, 0x01, 0xF5, 0xC0, 0xFF, 0x08, 0xFC, 0x00, 0xE5,
	0x0B, 0xF8, 0xC2, 0xCA, 0xF4, 0xF9, 0x0D, 0x7F, 0x44, 0x6D, 0x19, 0xFE, 0x46, 0x97, 0x33, 0x5E,
	0x08, 0xFF, 0xD1, 0xFF, 0xC6, 0x8B, 0x24, 0x74, 0x12, 0xFC, 0x00, 0x9F, 0x94, 0xB7, 0x06, 0xD5,
	0x40, 0x7A, 0x20, 0x9E, 0x04, 0x5F, 0x41, 0x2F, 0x3D, 0x77, 0x36, 0x75, 0x81, 0x8A, 0x70, 0x3A,
	0x98, 0xD1, 0x71, 0x02, 0x4D, 0x01, 0xC1, 0xFF, 0x0D, 0x00, 0xD3, 0x05, 0xF9, 0x00, 0x0B, 0x00,
];

const SPRITES_START_INDEX: u16 = 0xFE00;
const JOYPAD_INDEX: u16 = 0xFF00;
const DIVIDER_INDEX: u16 = 0xFF04;
const SELECTABLE_TIMER_INDEX: u16 = 0xFF05;
const TIMER_RESET_INDEX: u16 = 0xFF06;
const TIMER_CONTROL_INDEX: u16 = 0xFF07;
const INTERRUPT_FLAGS_INDEX: u16 = 0xFF0F;
const APU_INDEX_START: u16 = 0xFF10;
const APU_INDEX_END: u16 = 0xFF3F;
const LCD_CONTROL_INDEX: u16 = 0xFF40;
const LCD_INDEX: u16 = 0xFF41;
const SCROLL_Y_INDEX: u16 = 0xFF42;
const SCROLL_X_INDEX: u16 = 0xFF43;
const LY_INDEX: u16 = 0xFF44;
const LYC_INDEX: u16 = 0xFF45;
const BACKGROUND_PALETTE_INDEX: u16 = 0xFF47;
const OBJECT_PALETTE_0_INDEX: u16 = 0xFF48;
const OBJECT_PALETTE_1_INDEX: u16 = 0xFF49;
const WINDOW_Y_INDEX: u16 = 0xFF4A;
const WINDOW_X_INDEX: u16 = 0xFF4B;
const VRAM_BANK_INDEX: u16 = 0xFF4F;
const CGB_BACKGROUND_PALETTE_INDEX_INDEX: u16 = 0xFF68;
const CGB_BACKGROUND_PALETTE_DATA_INDEX: u16 = 0xFF69;
const CGB_SPRITE_PALETTE_INDEX_INDEX: u16 = 0xFF6A;
const CGB_SPRITE_PALETTE_DATA_INDEX: u16 = 0xFF6B;
const INTERRUPT_ENABLE_INDEX: u16 = 0xFFFF;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
	Vblank = 0,
	Lcd = 1,
	Timer = 2,
	Serial = 3,
	Joypad = 4,
}

pub struct Memory {
	mbc: Box<dyn Mbc>,
	wram: Vec<u8>,
	vram: Vec<u8>,
	hram: [u8; 0x100],
	oam: [u8; 0x100],

	joypad_state: u8,
	pub scan_line: u8,
	pub irq48_signal: u8,
	pub screen_disabled: bool,
	pub lcd_status_mode: u8,
	gpu_cycles: GpuCycles,
	pub div_cycles: i32,
	pub tima_cycles: i32,
	is_cgb: bool,
	vram_bank: i32,
	wram_bank: i32,
	hdma_source: u16,
	hdma_destination: u16,
	hdma_bytes: i32,
	hdma_enabled: bool,
	pub cgb_background_palettes: [[Color; 4]; 8],
	pub cgb_sprite_palettes: [[Color; 4]; 8],
	audio: Audio,
}

impl Memory {
	pub fn from_cartridge(cartridge: Cartridge, is_cgb: bool) -> Self {
		let hram = if is_cgb {
			INITIAL_VALUES_FOR_COLOR_FFXX
		} else {
			INITIAL_VALUES_FOR_FFXX
		};

		let mbc: Box<dyn Mbc> = match cartridge.get_mbc_type() {
			MbcType::RomOnly => Box::new(RomOnly::new(cartridge)),
			MbcType::Mbc1 => Box::new(Mbc1::new(cartridge)),
			other => panic!("{:?}", other),
		};

		let vram = if is_cgb {
			vec![0x00; 0x2000 * 2]
		} else {
			vec![0x00; 0x2000]
		};

		let wram = if is_cgb {
			vec![0x00; 0x1000 * 8]
		} else {
			vec![0x00; 0x1000 * 2]
		};

		let mut hdma_source = 0;
		let mut hdma_destination = 0;
		if is_cgb {
			// TODO: make all of these MMU constants
			let mut hdma_source_high = hram[0xFF51 - 0xFF00] as u16;
			let hdma_source_low = hram[0xFF52 - 0xFF00] as u16;
			if hdma_source_high > 0x7F && hdma_source_high < 0xA0 {
				hdma_source_high = 0;
			}
			hdma_source = (hdma_source_high << 8) | (hdma_source_low & 0xF0);
			let hdma_destination_high = hram[0xFF53 - 0xFF00] as u16;
			let hdma_destination_low = hram[0xFF54 - 0xFF00] as u16;
			hdma_destination =
				((hdma_destination_high & 0x1F) << 8) | (hdma_destination_low & 0xF0);
			hdma_destination |= 0x8000;
		}

		// setup initial values for the sound module
		let mut audio = Audio::new();
		// for i in 0xFF10..=0xFF3F {
		// 	let value = if is_cgb {
		// 		INITIAL_VALUES_FOR_COLOR_FFXX[i - 0xFF00]
		// 	} else {
		// 		INITIAL_VALUES_FOR_FFXX[i - 0xFF00]
		// 	};
		// 	audio.write_byte(i as u16, value);
		// }

		Self {
			mbc,
			vram,
			wram,
			hram,
			oam: [0; 0x100],
			scan_line: 144,
			joypad_state: 0,
			irq48_signal: 0,
			screen_disabled: false,
			lcd_status_mode: 1,
			gpu_cycles: GpuCycles::new(),
			div_cycles: 0,
			tima_cycles: 0,
			is_cgb,
			vram_bank: 0,
			wram_bank: 1,
			hdma_source,
			hdma_destination,
			hdma_bytes: 0,
			hdma_enabled: false,
			cgb_background_palettes: [[Color::WHITE; 4]; 8],
			cgb_sprite_palettes: [[Color::WHITE; 4]; 8],
			audio,
		}
	}

	pub fn read_byte(&self, index: u16) -> u8 {
		match index {
			0x0000..=0x7FFF => self.mbc.read_byte(index),
			0x8000..=0x9FFF => self.read_cgb_lcd_ram(index, self.vram_bank),
			0xA000..=0xBFFF => self.mbc.read_byte(index),
			0xC000..=0xCFFF => self.read_cgb_wram(index - 0xC000, 0),
			0xD000..=0xDFFF => self.read_cgb_wram(index - 0xD000, self.wram_bank),
			0xE000..=0xFDFF => self.read_byte(index - 0x2000),
			0xFE00..=0xFEFF => self.oam[index as usize - 0xFE00],

			JOYPAD_INDEX => todo!(),
			0xFF03 => 0xFF,
			DIVIDER_INDEX => self.load(index),
			SELECTABLE_TIMER_INDEX => self.load(index),
			TIMER_RESET_INDEX => self.load(index),
			TIMER_CONTROL_INDEX => self.load(index) | 0xF8,
			0xFF0E => 0xFF,
			INTERRUPT_FLAGS_INDEX => self.load(index) | 0xE0,
			APU_INDEX_START..=APU_INDEX_END => todo!(),
			LCD_CONTROL_INDEX => self.load(index),
			LCD_INDEX => self.load(index) | 0x80,
			SCROLL_Y_INDEX => self.load(index),
			SCROLL_X_INDEX => self.load(index),
			LY_INDEX => {
				if !self.screen_disabled {
					self.scan_line
				} else {
					0x00
				}
			}
			LYC_INDEX => self.load(index),
			WINDOW_Y_INDEX => self.load(index),
			WINDOW_X_INDEX => self.load(index),
			0xFF4C => 0xFF,
			BACKGROUND_PALETTE_INDEX => self.load(index),
			OBJECT_PALETTE_0_INDEX => self.load(index),
			OBJECT_PALETTE_1_INDEX => self.load(index),
			VRAM_BANK_INDEX => self.load(index) | 0xFE,
			CGB_BACKGROUND_PALETTE_INDEX_INDEX | CGB_SPRITE_PALETTE_INDEX_INDEX => {
				if self.is_cgb {
					self.load(index) | 0x40
				} else {
					0xC0
				}
			}
			CGB_BACKGROUND_PALETTE_DATA_INDEX | CGB_SPRITE_PALETTE_DATA_INDEX => {
				if self.is_cgb {
					self.load(index) | 0xF8
				} else {
					0xFF
				}
			}
			0xFF70 => {
				if self.is_cgb {
					self.load(index) | 0x40
				} else {
					0xFF
				}
			}
			0xFF76 => {
				if self.is_cgb {
					0x00
				} else {
					0xFF
				}
			}
			0xFF77 => {
				if self.is_cgb {
					0x00
				} else {
					0xFF
				}
			}

			_ => self.load(index),

			_ => panic!("tried to read: {:04X}", index),
		}
	}

	pub fn write_byte(&mut self, index: u16, val: u8) {
		match index {
			0x0000..=0x7FFF => self.mbc.write_byte(index, val),
			0x8000..=0x9FFF => self.write_cgb_lcd_ram(index, val, self.vram_bank),
			0xA000..=0xBFFF => self.mbc.write_byte(index, val),
			0xC000..=0xCFFF => self.write_cgb_wram(index - 0xC000, val, 0),
			0xD000..=0xDFFF => self.write_cgb_wram(index - 0xD000, val, self.wram_bank),
			0xE000..=0xFDFF => self.write_byte(index - 0x2000, val),
			0xFE00..=0xFEFF => self.oam[index as usize - 0xFE00] = val,

			0xFF02 => {
				if val == 0x81 {
					println!("{}", self.read_byte(0xFF01) as char);
				}
			}

			DIVIDER_INDEX => self.store(index, 0),
			SELECTABLE_TIMER_INDEX => self.store(index, val),
			TIMER_RESET_INDEX => self.store(index, val),
			TIMER_CONTROL_INDEX => self.store(index, val),
			INTERRUPT_FLAGS_INDEX => self.store(index, val & 0x1F),
			APU_INDEX_START..=APU_INDEX_END => (), // TODO
			LCD_CONTROL_INDEX => self.do_lcd_control_write(val),
			LCD_INDEX => self.do_lcd_status_write(val),
			SCROLL_Y_INDEX => self.store(index, val),
			SCROLL_X_INDEX => self.store(index, val),
			LY_INDEX => self.do_scanline_write(val),
			LYC_INDEX => self.do_lyc_write(val),
			0xFF46 => {
				self.store(index, val);
				self.do_dma_transfer(val)
			}
			BACKGROUND_PALETTE_INDEX => self.store(index, val),
			OBJECT_PALETTE_0_INDEX => self.store(index, val),
			OBJECT_PALETTE_1_INDEX => self.store(index, val),
			WINDOW_Y_INDEX => self.store(index, val),
			WINDOW_X_INDEX => self.store(index, val),
			0xFF4D if self.is_cgb => {
				let current_key1 = self.load(index);
				self.store(index, (current_key1 & 0x80) | (val & 1) | 0x7E);
			}
			VRAM_BANK_INDEX if self.is_cgb => {
				let value = val & 1;
				self.vram_bank = value as i32;
				self.store(index, value);
			}

			CGB_BACKGROUND_PALETTE_INDEX_INDEX => {
				self.store(index, val);
			}
			CGB_BACKGROUND_PALETTE_DATA_INDEX => {
				self.store(index, val);
			}
			CGB_SPRITE_PALETTE_INDEX_INDEX => {
				self.store(index, val);
			}
			CGB_SPRITE_PALETTE_DATA_INDEX => {
				self.store(index, val);
			}
			0xFF6C => self.store(0xFF6C, val | 0xFE),
			0xFF75 => self.store(0xFF75, val | 0x8F),
			INTERRUPT_ENABLE_INDEX => self.store(index, val & 0x1F),

			_ => self.store(index, val),
			_ => panic!("tried to write: {:04X}", index),
		}
	}

	pub fn read_word(&self, index: u16) -> u16 {
		let low = self.read_byte(index) as u16;
		let high = self.read_byte(index + 1) as u16;
		(high << 8) + low
	}

	pub fn write_word(&mut self, index: u16, value: u16) {
		let high = (value >> 8) as u8;
		let low = value as u8;
		self.write_byte(index, low);
		self.write_byte(index.wrapping_add(1), high);
	}

	fn read_cgb_wram(&self, index: u16, bank: i32) -> u8 {
		let offset = 0x1000 * bank as usize;
		let address = index as usize + offset;
		self.wram[address]
	}

	fn write_cgb_wram(&mut self, index: u16, val: u8, bank: i32) {
		let offset = 0x1000 * bank as usize;
		let address = index as usize + offset;
		self.wram[address] = val;
	}

	fn load(&self, addr: u16) -> u8 {
		self.hram[addr as usize - 0xFF00]
	}

	fn store(&mut self, addr: u16, val: u8) {
		self.hram[addr as usize - 0xFF00] = val;
	}

	// TODO: check specific control flags
	fn do_lcd_control_write(&mut self, val: u8) {
		self.store(LCD_CONTROL_INDEX, val);
	}

	fn do_lcd_status_write(&mut self, val: u8) {
		let current_stat = self.load(LCD_INDEX) & 0x07;
		let new_stat = (val & 0x78) | (current_stat & 0x07);
		self.store(LCD_INDEX, new_stat);
		let lcd_control = LcdControlFlag::from_bits_truncate(self.load(LCD_CONTROL_INDEX));
		let mut signal = self.irq48_signal;
		let mode = self.lcd_status_mode;
		signal &= (new_stat >> 3) & 0x0F;
		self.irq48_signal = signal;

		if lcd_control.contains(LcdControlFlag::DISPLAY) {
			if is_set(new_stat, 3) && mode == 0 {
				if signal == 0 {
					self.request_interrupt(Interrupt::Lcd);
				}
				signal = set_bit(signal, 0);
			}

			if is_set(new_stat, 4) && mode == 1 {
				if signal == 0 {
					self.request_interrupt(Interrupt::Lcd);
				}
				signal = set_bit(signal, 1);
			}

			if is_set(new_stat, 5) && mode == 2 && signal == 0 {
				self.request_interrupt(Interrupt::Lcd);
			}
			self.compare_ly_to_lyc();
		}
	}

	pub fn request_interrupt(&mut self, interrupt: Interrupt) {
		let mut interrupt_flag = self.read_byte(INTERRUPT_FLAGS_INDEX);
		let interrupt = interrupt as u8;
		interrupt_flag = set_bit(interrupt_flag, interrupt);
		self.write_byte(INTERRUPT_FLAGS_INDEX, interrupt_flag);
	}

	fn do_dma_transfer(&mut self, data: u8) {
		let address = 0x100 * u16::from(data);
		if address >= 0x8000 && address < 0xE000 {
			for i in 0..0xA0 {
				let value = self.read_byte(address + i);
				self.write_byte(SPRITES_START_INDEX + i, value);
			}
		}
	}

	fn read_cgb_lcd_ram(&self, index: u16, bank: i32) -> u8 {
		let offset = 0x2000 * bank as usize;
		let address = index as usize - 0x8000 + offset;
		self.vram[address]
	}

	fn do_scanline_write(&mut self, val: u8) {
		let current_ly = self.scan_line;
		if is_set(current_ly, 7) && !is_set(val, 7) {
			self.disable_screen();
		}
	}

	fn disable_screen(&mut self) {
		self.screen_disabled = true;
		let mut stat = self.load(LCD_INDEX);
		stat &= 0x7C;
		self.store(LCD_INDEX, stat);
		self.lcd_status_mode = 0;
		// self.gpu_cycles.cycles_counter = 0;
		// self.gpu_cycles.aux_cycles_counter = 0;
		self.scan_line = 0;
		self.irq48_signal = 0;
	}

	fn do_lyc_write(&mut self, val: u8) {
		let current_lyc = self.load(LYC_INDEX);
		if current_lyc != val {
			self.store(LYC_INDEX, val);
			let lcd_control = LcdControlFlag::from_bits_truncate(self.load(LCD_CONTROL_INDEX));
			if lcd_control.contains(LcdControlFlag::DISPLAY) {
				self.compare_ly_to_lyc();
			}
		}
	}

	fn compare_ly_to_lyc(&mut self) {
		if !self.screen_disabled {
			let lyc = self.load(LYC_INDEX);
			let mut stat = self.load(LCD_INDEX);

			if lyc == self.scan_line {
				stat = set_bit(stat, 2);
				if is_set(stat, 6) {
					if self.irq48_signal == 0 {
						self.request_interrupt(Interrupt::Lcd);
					}
					self.irq48_signal = set_bit(self.irq48_signal, 3);
				}
			} else {
				stat = unset_bit(stat, 2);
				self.irq48_signal = unset_bit(self.irq48_signal, 3);
			}
			self.store(LCD_INDEX, stat);
		}
	}

	fn write_cgb_lcd_ram(&mut self, index: u16, val: u8, bank: i32) {
		let offset = 0x2000 * bank as usize;
		let address = index as usize - 0x8000 + offset;
		self.vram[address] = val;
	}
}

struct GpuCycles {}

impl GpuCycles {
	pub fn new() -> Self {
		Self {}
	}
}

bitflags::bitflags! {
	pub struct LcdControlFlag : u8 {
		const BACKGROUND            = 0b0000_0001;
		const SPRITES               = 0b0000_0010;
		const SPRITES_SIZE          = 0b0000_0100;
		const BACKGROUND_TILE_MAP   = 0b0000_1000;
		const BACKGROUND_TILE_SET   = 0b0001_0000;
		const WINDOW                = 0b0010_0000;
		const WINDOW_TILE_MAP       = 0b0100_0000;
		const DISPLAY               = 0b1000_0000;
	}
}
