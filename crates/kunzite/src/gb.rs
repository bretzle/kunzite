use crate::{cpu::Cpu, display::Display, memory::Memory};

pub struct Gb {
	cpu: Cpu,
	memory: Memory,
	display: Display,
}
