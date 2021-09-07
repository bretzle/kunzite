//!

use crate::{cpu::Cpu, display::Display, memory::Memory};

/// Brings all the components into a single package
pub struct Gb {
	cpu: Cpu,
	memory: Memory,
	display: Display,
}
