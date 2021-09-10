//! Memory module

use std::ops::{Index, IndexMut};

/// Memory
pub struct Memory([u8; 0x10000]);

impl Memory {
	/// Create a new memory instance
	pub fn new() -> Self {
		Self([0; 0x10000])
	}
}

impl Index<usize> for Memory {
	type Output = u8;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl IndexMut<usize> for Memory {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}
