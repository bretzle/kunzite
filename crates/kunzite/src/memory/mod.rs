//! Memory module

/// Memory
pub struct Memory([u8; 0x10000]);

impl Memory {
	/// Create a new memory instance
	pub fn new() -> Self {
		Self([0; 0x10000])
	}
}
