//! The cpu

use std::{fs::File, io::Read, ops::BitXor, path::Path};

use color_eyre::Result;

use crate::{
	cpu::instruction::{Flag, Instruction, Register16},
	memory::Memory,
	util::{lower, upper},
};

use self::{instruction::Register8, register::Registers};

mod decode;
pub mod instruction;
mod register;

/// The cpu
#[derive(Default)]
pub struct Cpu {
	rom: Option<Vec<u8>>,
	/// Program counter
	pub pc: u16,
	/// Cpu registers
	pub registers: Registers,
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
	pub fn step(&mut self, memory: &mut Memory) {
		if let Some((_, inst)) = self.parse_instruction() {
			self.pc += inst.size();

			macro_rules! update_flags {
				(
					$( z: $z:expr, )?
					$( n: $n:expr, )?
					$( h: $h:expr, )?
					$( c: $c:expr, )?
				) => {
					$( self.registers.set_flag(Flag::Z, $z as u8); )?
					$( self.registers.set_flag(Flag::N, $n as u8); )?
					$( self.registers.set_flag(Flag::H, $h as u8); )?
					$( self.registers.set_flag(Flag::C, $c as u8); )?
				}
			}

			match inst {
				Instruction::Nop => todo!("{:?}", inst),
				Instruction::Stop => todo!("{:?}", inst),
				Instruction::Halt => todo!("{:?}", inst),
				Instruction::StoreImm16(reg, val) => {
					self.registers[reg] = val;
				}
				Instruction::StoreImm8(reg, val) => {
					self.registers[reg] = val;
				}
				Instruction::StoreAToHlAddr(inc) => {
					let hl = self.registers[Register16::HL];
					memory[hl as usize] = self.registers[Register8::A];

					let hl = &mut self.registers[Register16::HL];
					if inc {
						*hl += 1;
					} else {
						*hl -= 1;
					}
				}
				Instruction::LoadAFromHlAddr(_) => todo!("{:?}", inst),
				Instruction::StoreATo16(_) => todo!("{:?}", inst),
				Instruction::LoadAFromReg16Addr(reg) => {
					let addr = self.registers[reg];
					self.registers[Register8::A] = memory[addr as usize];
				}
				Instruction::Mov8(dest, src) => {
					let val = self.registers[src];
					match dest {
						Register8::DerefHL => memory[self.registers[Register16::HL] as usize] = val,
						_ => self.registers[dest] = val,
					}
				}
				Instruction::Jr(f, r) => match f {
					Some(flag) => {
						if self.registers.flag(flag) {
							self.pc = ((self.pc) as i16 + r as i16) as u16
						}
					}
					None => self.pc = ((self.pc - inst.size()) as i16 + r as i16) as u16,
				},
				Instruction::Jp(_, _) => todo!("{:?}", inst),
				Instruction::Inc8(reg) => {
					let (new, _) = self.registers[reg].overflowing_add(1);
					update_flags! {
						z: new == 0,
						n: 0,
						h: (self.registers[reg] & 0xF) + 1 > 0xF,
					};
				}
				Instruction::Dec8(_) => todo!("{:?}", inst),
				Instruction::Inc16(_) => todo!("{:?}", inst),
				Instruction::Dec16(_) => todo!("{:?}", inst),
				Instruction::Push(reg) => {
					let regs = reg.tear();
					self.push(memory, self.registers[regs.0]);
					self.push(memory, self.registers[regs.1]);
				}
				Instruction::Pop(reg) => {
					let regs = reg.tear();
					self.registers[regs.1] = self.pop(memory);
					self.registers[regs.0] = self.pop(memory);
				}
				Instruction::Add(_) => todo!("{:?}", inst),
				Instruction::Adc(_) => todo!("{:?}", inst),
				Instruction::Sub(_) => todo!("{:?}", inst),
				Instruction::Sbc(_) => todo!("{:?}", inst),
				Instruction::And(_) => todo!("{:?}", inst),
				Instruction::Xor(reg) => {
					self.registers[Register8::A] ^= self.registers[reg];
					update_flags! {
						z: self.registers[Register8::A] == 0,
						n: 0,
						h: 0,
						c: 0,
					}
				}
				Instruction::Or(_) => todo!("{:?}", inst),
				Instruction::Cp(_) => todo!("{:?}", inst),
				Instruction::Add8(_) => todo!("{:?}", inst),
				Instruction::Adc8(_) => todo!("{:?}", inst),
				Instruction::Sub8(_) => todo!("{:?}", inst),
				Instruction::Sbc8(_) => todo!("{:?}", inst),
				Instruction::And8(_) => todo!("{:?}", inst),
				Instruction::Xor8(_) => todo!("{:?}", inst),
				Instruction::Or8(_) => todo!("{:?}", inst),
				Instruction::Cp8(_) => todo!("{:?}", inst),
				Instruction::AddSp8(_) => todo!("{:?}", inst),
				Instruction::Daa => todo!("{:?}", inst),
				Instruction::Scf => todo!("{:?}", inst),
				Instruction::Cpl => todo!("{:?}", inst),
				Instruction::Ccf => todo!("{:?}", inst),
				Instruction::Rlca => todo!("{:?}", inst),
				Instruction::Rla => todo!("{:?}", inst),
				Instruction::Rrca => todo!("{:?}", inst),
				Instruction::Rra => todo!("{:?}", inst),
				Instruction::StoreImm16AddrSp(_) => todo!("{:?}", inst),
				Instruction::AddHl(_) => todo!("{:?}", inst),
				Instruction::Ret(_) => todo!("{:?}", inst),
				Instruction::Reti => todo!("{:?}", inst),
				Instruction::Di => todo!("{:?}", inst),
				Instruction::Ei => todo!("{:?}", inst),
				Instruction::Call(f, jump) => match f {
					Some(flag) => todo!("{:?}", inst),
					None => {
						self.push(memory, upper(self.pc));
						self.push(memory, lower(self.pc));
						self.pc = jump;
					}
				},
				Instruction::JpHl => todo!("{:?}", inst),
				Instruction::Rst(_) => todo!("{:?}", inst),
				Instruction::LdHlSp8(_) => todo!("{:?}", inst),
				Instruction::LdSpHl => todo!("{:?}", inst),
				Instruction::StoreHA(offset) => {
					let addr = 0xFF00 + offset as u16;
					memory[addr as usize] = self.registers[Register8::A];
				}
				Instruction::LoadHA(_) => todo!("{:?}", inst),
				Instruction::StoreCA => {
					let addr = 0xFF00 + self.registers[Register8::C] as u16;
					memory[addr as usize] = self.registers[Register8::A];
				}
				Instruction::LoadCA => todo!("{:?}", inst),
				Instruction::StoreAAtAddress(_) => todo!("{:?}", inst),
				Instruction::LoadAFromAddress(_) => todo!("{:?}", inst),
				Instruction::Rlc(_) => todo!("{:?}", inst),
				Instruction::Rrc(_) => todo!("{:?}", inst),
				Instruction::Rr(_) => todo!("{:?}", inst),
				Instruction::Rl(reg) => match reg {
					Register8::DerefHL => todo!(),
					reg => {
						let overflow = (self.registers[reg] & 0x80) != 0;
						self.registers[reg] <<= 1;
						let new = self.registers[reg];

						update_flags! {
							z: new == 0,
							n: 0,
							h: 0,
							c: overflow,
						}
					}
				},
				Instruction::Sla(_) => todo!("{:?}", inst),
				Instruction::Sra(_) => todo!("{:?}", inst),
				Instruction::Swap(_) => todo!("{:?}", inst),
				Instruction::Srl(_) => todo!("{:?}", inst),
				Instruction::Bit(bit, reg) => {
					let set = self.registers[reg] & (1 << bit) != 0;
					update_flags! {
						z: !set,
						n: 0,
						h: 1,
					}
				}
				Instruction::Res(_, _) => todo!("{:?}", inst),
				Instruction::Set(_, _) => todo!("{:?}", inst),
				/* Instruction::Jp(f, addr) => match f {
				 * 	Some(flag) => {
				 * 		if self.registers.flag(flag) {
				 * 			self.pc = addr;
				 * 		}
				 * 	}
				 * 	None => self.pc = addr,
				 * },
				 * } */
			}
		}
	}

	fn push(&mut self, memory: &mut Memory, val: u8) {
		let sp = &mut self.registers[Register16::SP];

		memory[*sp as usize] = val;

		*sp -= 1;
	}

	fn pop(&mut self, memory: &mut Memory) -> u8 {
		let sp = &mut self.registers[Register16::SP];

		let val = memory[*sp as usize];

		*sp += 1;

		val
	}
}

#[test]
fn feature() {
	let a = 0x80u8;

	assert_eq!(a.overflowing_shl(1), (0, true));
}
