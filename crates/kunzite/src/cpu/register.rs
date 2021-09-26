use std::fmt::{Debug, UpperHex};

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Flag(u8);

impl Flag {
	pub const FULL_CARRY: Self = Self(0b0001_0000);
	pub const HALF_CARRY: Self = Self(0b0010_0000);
	pub const NEGATIVE: Self = Self(0b0100_0000);
	pub const NOT_FULL_CARRY: Self = Self(0b0000_0001);
	pub const NOT_HALF_CARRY: Self = Self(0b0000_0010);
	pub const NOT_NEGATIVE: Self = Self(0b0000_0100);
	pub const NOT_ZERO: Self = Self(0b0000_1000);
	pub const ZERO: Self = Self(0b1000_0000);

	pub fn contains(&self, flag: Flag) -> bool {
		match flag {
			Flag::ZERO => crate::util::is_set(self.0, 7),
			Flag::NEGATIVE => crate::util::is_set(self.0, 6),
			Flag::HALF_CARRY => crate::util::is_set(self.0, 5),
			Flag::FULL_CARRY => crate::util::is_set(self.0, 4),
			Flag::NOT_ZERO => !crate::util::is_set(self.0, 7),
			Flag::NOT_NEGATIVE => !crate::util::is_set(self.0, 6),
			Flag::NOT_HALF_CARRY => !crate::util::is_set(self.0, 5),
			Flag::NOT_FULL_CARRY => !crate::util::is_set(self.0, 4),
			_ => unreachable!(),
		}
	}

	pub fn new(val: u8) -> Self {
		Self(val & 0xF0)
	}

	pub fn bits(&self) -> u8 {
		self.0
	}

	pub fn set(&mut self, bit: u8, val: bool) {
		self.0 = if val {
			crate::util::set_bit(self.0, bit)
		} else {
			crate::util::unset_bit(self.0, bit)
		}
	}
}

impl Debug for Flag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			&Flag::ZERO => write!(f, "Z"),
			&Flag::NEGATIVE => write!(f, "N"),
			&Flag::HALF_CARRY => write!(f, "H"),
			&Flag::FULL_CARRY => write!(f, "C"),
			_ => panic!(),
		}
	}
}

impl UpperHex for Flag {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:X}", self.0)
	}
}

#[derive(Default)]
pub struct Registers {
	pub a: u8,
	pub f: Flag,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub h: u8,
	pub l: u8,
	pub pc: u16,
	pub sp: u16,
}

impl Registers {
	pub fn get_af(&self) -> u16 {
		(u16::from(self.a) << 8) | u16::from(self.f.bits())
	}

	pub fn set_af(&mut self, af: u16) {
		self.a = (af >> 8) as u8;
		self.f = Flag::new(af as u8);
	}

	pub fn get_bc(&self) -> u16 {
		(u16::from(self.b) << 8) | u16::from(self.c)
	}

	pub fn set_bc(&mut self, bc: u16) {
		self.c = bc as u8;
		self.b = (bc >> 8) as u8;
	}

	pub fn get_de(&self) -> u16 {
		(u16::from(self.d) << 8) | u16::from(self.e)
	}

	pub fn set_de(&mut self, de: u16) {
		self.e = de as u8;
		self.d = (de >> 8) as u8;
	}

	pub fn get_hl(&self) -> u16 {
		(u16::from(self.h) << 8) | u16::from(self.l)
	}

	pub fn set_hl(&mut self, hl: u16) {
		self.l = hl as u8;
		self.h = (hl >> 8) as u8;
	}
}
