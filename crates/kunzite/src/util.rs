pub fn upper(val: u16) -> u8 {
	(val >> 8) as u8
}

pub fn lower(val: u16) -> u8 {
	(val & 0xFF) as u8
}

pub fn slice_to_string(slice: &[u8]) -> String {
	slice
		.iter()
		.filter(|b| **b != 0)
		.map(|b| *b as char)
		.collect()
}

pub fn is_set(byte: u8, bit: u8) -> bool {
	byte & (1 << bit) != 0
}

pub fn set_bit(byte: u8, bit: u8) -> u8 {
	byte | (1 << bit)
}

pub fn unset_bit(byte: u8, bit: u8) -> u8 {
	byte & !(1 << bit)
}

#[derive(Clone, Copy)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Color {
	pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
	pub const WHITE: Self = Self {
		r: 255,
		g: 255,
		b: 255,
	};
}
