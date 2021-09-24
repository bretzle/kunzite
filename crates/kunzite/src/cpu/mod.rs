//! The cpu

pub use self::register::Flag;
use self::register::Registers;
use crate::memory::Memory;

mod decode;
pub mod instruction;
mod register;

/// The cpu
pub struct Cpu {
	registers: Registers,
	halted: bool,
	interrupt_enabled: bool,
	instruction_cycle: i32,
	is_cgb: bool,
}

impl Cpu {
	pub fn new(is_cgb: bool) -> Self {
		let mut registers = Registers::default();

		registers.set_af(0x01B0);
		registers.set_bc(0x0013);
		registers.set_de(0x00D8);
		registers.set_hl(0x014D);
		registers.pc = 0x0100;
		registers.sp = 0xFFFE;

		Self {
			registers,
			halted: false,
			interrupt_enabled: false,
			instruction_cycle: 0,
			is_cgb,
		}
	}

	pub fn step(&mut self, memory: &mut Memory) -> i32 {
		self.instruction_cycle = 0;

		if !self.halted {
			// TODO: handle interrupts

			let opcode = self.fetch(memory);
			self.execute(opcode, memory)
		} else {
			panic!("Support halts")
		}

		self.instruction_cycle
	}

	fn execute(&mut self, opcode: u8, memory: &mut Memory) {
		todo!()
	}

	pub fn pc(&self) -> &u16 {
		&self.registers.pc
	}

	pub fn regs(&self) -> &Registers {
		&self.registers
	}

	pub fn pc_mut(&mut self) -> &mut u16 {
		&mut self.registers.pc
	}
}
