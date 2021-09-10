use std::ops::{Index, IndexMut};

use super::instruction::{Flag, Register8};

#[derive(Default)]
pub struct Registers {
	a: u8,
	b: u8,
	c: u8,
	d: u8,
	e: u8,
	f: u8,
	h: u8,
	l: u8,
}

impl Registers {
	pub fn flag(&self, flag: Flag) -> bool {
		match flag {
			Flag::Z => self.f & 0x80 != 0,
			Flag::NZ => self.f & 0x80 == 0,
			Flag::N => self.f & 0x40 != 0,
			Flag::H => self.f & 0x20 != 0,
			Flag::C => self.f & 0x10 != 0,
			Flag::NC => self.f & 0x10 == 0,
		}
	}

	pub fn set_flag(&mut self, flag: Flag, val: u8) {
		debug_assert!(val == 0 || val == 1);

		match flag {
			Flag::Z => {
				self.f &= 0x7F;
				self.f |= val << 7;
			}
			Flag::N => {
				self.f &= 0xBF;
				self.f |= val << 6;
			}
			Flag::H => {
				self.f &= 0xDF;
				self.f |= val << 5;
			}
			Flag::C => {
				self.f &= 0xEF;
				self.f |= val << 4;
			}
			_ => unreachable!(),
		}
	}
}

impl Index<Register8> for Registers {
	type Output = u8;

	fn index(&self, index: Register8) -> &Self::Output {
		match index {
			Register8::A => &self.a,
			Register8::B => &self.b,
			Register8::C => &self.c,
			Register8::D => &self.d,
			Register8::E => &self.e,
			Register8::H => &self.h,
			Register8::L => &self.l,
			Register8::DerefHL => todo!(),
		}
	}
}

impl IndexMut<Register8> for Registers {
	fn index_mut(&mut self, index: Register8) -> &mut Self::Output {
		match index {
			Register8::A => &mut self.a,
			Register8::B => &mut self.b,
			Register8::C => &mut self.c,
			Register8::D => &mut self.d,
			Register8::E => &mut self.e,
			Register8::H => &mut self.h,
			Register8::L => &mut self.l,
			Register8::DerefHL => todo!(),
		}
	}
}
