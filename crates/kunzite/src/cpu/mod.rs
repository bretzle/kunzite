//! The cpu

use std::{fs::File, io::Read, path::Path};

use color_eyre::Result;

use crate::cpu::instruction::{Flag, Instruction};

use self::{instruction::Register8, register::Registers};

mod decode;
mod instruction;
mod register;

/// The cpu
#[derive(Default)]
pub struct Cpu {
	rom: Option<Vec<u8>>,
	pc: u16,
	registers: Registers,
}

impl Cpu {
	/// Insert a cartridge into the cpu
	pub fn insert_rom<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
		let mut file = File::open(path)?;

		let size = file.read_to_end(self.rom.insert(vec![]))?;

		println!("Rom size: {} bytes", size);

		Ok(())
	}

	/// Execute an instruction and increment pc
	pub fn step(&mut self) {
		if let Some((_, inst)) = self.parse_instruction() {
			self.pc += inst.size();

			macro_rules! update_flags {
				(
					$( z: $z:expr, )?
					$( n: $n:expr, )?
					$( h: $h:expr, )?
					$( c: $c:expr, )?
				) => {
					$( self.registers.set_flag(Flag::Z, $z) )?
				}
			}

			match dbg!(inst) {
				Instruction::Inc8(reg) => {
					let (new, _) = self.registers[reg].overflowing_add(1);
					update_flags! {
						z: (new == 0) as u8,
						n: 0,
						h: (self.registers[reg] & 0xF) + 1 > 0xF,
					};
				}
				Instruction::Jp(f, addr) => match f {
					Some(flag) => {
						if self.registers.flag(flag) {
							self.pc = addr;
						}
					}
					None => self.pc = addr,
				},
				other => panic!("{:?} is not supported yet", other),
			}
		}
	}
}
