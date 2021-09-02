#![feature(exclusive_range_pattern)]

mod cpu;
mod display;
mod emulator;
mod gb;
mod memory;

use std::fmt::Debug;

use color_eyre::Result;
use emulator::Emulator;
use gui::*;

#[allow(clippy::many_single_char_names)]
fn main() -> Result<()> {
	color_eyre::install()?;

	// let options = Options::new("GB Emulator", 600, 400);

	// run::<Emulator>(options)

	const ROM: &[u8] = include_bytes!("../../../roms/bootloader.gb");
	let mut pc = 0;
	let mut insts = vec![];

	let mut prefix = Prefix::None;

	loop {
		if pc >= ROM.len() {
			break;
		}

		// there is static data here
		if pc == 0x00A8 {
			pc = 0x00E0;
		}

		let opcode = ROM[pc];

		let x = (opcode >> 6) & 0x3;
		let y = (opcode >> 3) & 0x7;
		let z = opcode & 0x7;
		let p = y >> 1;
		let q = y % 2;

		let d = ROM[pc + 1] as i8;
		let n = ROM[pc + 1];
		let nn = if pc + 2 < ROM.len() {
			((ROM[pc + 2] as u16) << 8) | n as u16
		} else {
			0
		};

		#[cfg(debug_assertions)]
		println!(
			"pc     = {:04X}\nopcode = {:#04X}\nx      = {:4}\ny      = {:4}\nz      = {:4}\np      = {:4}\nq      \
			 = {:4}\nd      = {:4}\nn      = {:04X}\nnn     = {:04X}\n-------------",
			pc, opcode, x, y, z, p, q, d, n, nn
		);

		const CC: [Flag; 4] = [Flag::NZ, Flag::Z, Flag::NC, Flag::C];
		const RP: [Reg; 4] = [Reg::BC, Reg::DE, Reg::HL, Reg::SP];
		const RP2: [Reg; 4] = [Reg::BC, Reg::DE, Reg::HL, Reg::AF];
		const R: [Reg; 8] = [
			Reg::B,
			Reg::C,
			Reg::D,
			Reg::E,
			Reg::H,
			Reg::L,
			Reg::HL,
			Reg::A,
		];

		match opcode {
			0xCB => {
				prefix = Prefix::CB;
				pc += 1;
				continue;
			}
			0xDD | 0xED | 0xFD => unreachable!(),
			_ => {}
		}

		let inst = match prefix {
			Prefix::None => match x {
				0 => match z {
					0 => match y {
						3 => Instruction::Jr(d),
						4..7 => Instruction::JrFlag(CC[y as usize - 4], d),
						_ => unreachable!(),
					},
					1 => match q {
						0 => Instruction::Ld16(RP[p as usize], nn),
						_ => unreachable!(),
					},
					2 => match q {
						0 => match p {
							2 => Instruction::Ldi(Reg::HL, Reg::A),
							3 => Instruction::Ldd(Reg::HL, Reg::A),
							_ => unreachable!(),
						},
						1 => match p {
							1 => Instruction::Ld(Reg::A, Reg::DE),
							_ => unreachable!(),
						},
						_ => unreachable!(),
					},
					3 => match q {
						0 => Instruction::Inc(RP[p as usize]),
						_ => unreachable!(),
					},
					4 => Instruction::Inc(R[y as usize]),
					5 => Instruction::Dec(R[y as usize]),
					6 => Instruction::Ld8(R[y as usize], n),
					7 => match y {
						2 => Instruction::Rla,
						_ => unreachable!(),
					},
					_ => unreachable!(),
				},
				1 => match z {
					6 => unreachable!(),
					_ => Instruction::Ld(R[y as usize], R[z as usize]),
				},
				2 => match y {
					0 => Instruction::Add(R[z as usize]),
					2 => Instruction::Sub(R[z as usize]),
					5 => Instruction::Xor(R[z as usize]),
					7 => Instruction::Cp(R[z as usize]),
					_ => unreachable!(),
				},
				3 => match z {
					0 => match y {
						4 => Instruction::LdAddr1(n),
						6 => Instruction::LdAddr3(n),
						_ => unreachable!(),
					},
					1 => match q {
						0 => Instruction::Pop(RP2[p as usize]),
						1 => match p {
							0 => Instruction::Ret,
							_ => unreachable!(),
						},
						_ => unreachable!(),
					},
					2 => match y {
						4 => Instruction::LdAddr2,
						5 => Instruction::LdAddr5(nn),
						_ => unreachable!(),
					},
					5 => match q {
						0 => Instruction::Push(RP2[p as usize]),
						1 => match p {
							0 => Instruction::Call(nn),
							1..3 => unreachable!("This opcode was removed: {:#0X}", opcode),
							_ => unreachable!(),
						},
						_ => unreachable!(),
					},
					6 => match y {
						1 => Instruction::Adc8(n),
						7 => Instruction::Cp8(n),
						_ => unreachable!(),
					},
					_ => unreachable!(),
				},
				_ => unreachable!(),
			},
			Prefix::CB => {
				pc -= 1;
				match x {
					0 => match y {
						2 => Instruction::Rl(R[z as usize]),
						_ => unreachable!(),
					},
					1 => Instruction::Bit(y, R[z as usize]),
					_ => unreachable!(),
				}
			}
		};

		prefix = Prefix::None;
		let size = inst.size();

		insts.push((pc, inst));
		pc += size;
	}

	for (pc, inst) in insts {
		println!("{:04X} {:?}", pc, inst);
	}

	Ok(())
}

enum Instruction {
	Ld16(Reg, u16),
	Ld8(Reg, u8),
	Ld(Reg, Reg),
	Ldi(Reg, Reg),
	Ldd(Reg, Reg),
	Xor(Reg),
	Bit(u8, Reg),
	JrFlag(Flag, i8),
	LdAddr1(u8), // opcode: 0xE0
	LdAddr2,     // opcode: 0xE2
	LdAddr3(u8), // opcode: 0xF0
	// LdAddr4,     // opcode: 0xF2
	Inc(Reg),
	Call(u16),
	Cp8(u8),
	Dec(Reg),
	LdAddr5(u16), // opcode: 0xEA
	// LdAddr6(u16), // opcode: 0xFA
	Jr(i8),
	Sub(Reg),
	Push(Reg),
	Rl(Reg),
	Rla,
	Pop(Reg),
	Ret,
	// Adc(Reg),
	Adc8(u8),
	Cp(Reg),
	Add(Reg),
}

impl Instruction {
	fn size(&self) -> usize {
		match self {
			Instruction::Ld16(_, _) => 3,
			Instruction::Ld8(_, _) => 2,
			Instruction::Ldi(_, _) => 1,
			Instruction::Ldd(_, _) => 1,
			Instruction::Xor(_) => 1,
			Instruction::Bit(_, _) => 2,
			Instruction::Jr(_) => 2,
			Instruction::JrFlag(_, _) => 2,
			Instruction::LdAddr2 => 1,
			Instruction::Inc(_) => 1,
			Instruction::Ld(_, _) => 1,
			Instruction::LdAddr1(_) => 2,
			Instruction::Call(_) => 3,
			Instruction::Cp8(_) => 2,
			Instruction::Dec(_) => 1,
			Instruction::LdAddr5(_) => 3,
			Instruction::LdAddr3(_) => 2,
			// Instruction::LdAddr4 => 1,
			Instruction::Sub(_) => 1,
			Instruction::Push(_) => 1,
			Instruction::Rl(_) => 2,
			Instruction::Rla => 1,
			Instruction::Pop(_) => 1,
			Instruction::Ret => 1,
			// Instruction::Adc(_) => 1,
			Instruction::Adc8(_) => 2,
			Instruction::Cp(_) => 1,
			Instruction::Add(_) => 1,
		}
	}
}

impl Debug for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Ld16(arg0, arg1) => write!(f, "LD   {:?}, ${:04X}", arg0, arg1),
			Self::Ld8(arg0, arg1) => write!(f, "LD   {:?}, ${:02X}", arg0, arg1),
			Self::Ld(arg0, arg1) => write!(f, "LD   {:?}, {:?}", arg0, arg1),
			Self::Ldi(arg0, arg1) => write!(f, "LD   {:?}+, {:?}", arg0, arg1),
			Self::Ldd(arg0, arg1) => write!(f, "LD   {:?}-, {:?}", arg0, arg1),
			Self::Xor(arg0) => write!(f, "XOR  {:?}", arg0),
			Self::Bit(arg0, arg1) => write!(f, "BIT  {}, {:?}", arg0, arg1),
			Self::JrFlag(arg0, arg1) => write!(f, "JR   {:?}, ${:02X}", arg0, arg1),
			Self::LdAddr1(arg0) => write!(f, "LD   ($FF00+${:02X}), A", arg0),
			Self::LdAddr2 => write!(f, "LD   ($FF00+C), A"),
			Self::LdAddr3(arg0) => write!(f, "LD   A, ($FF00+${:02X})", arg0),
			// Self::LdAddr4 => write!(f, "LD   A, ($FF00+C)"),
			Self::Inc(arg0) => write!(f, "INC  {:?}", arg0),
			Self::Call(arg0) => write!(f, "CALL ${:04X}", arg0),
			Self::Cp8(arg0) => write!(f, "CP   ${:02X}", arg0),
			Self::Dec(arg0) => write!(f, "DEC  {:?}", arg0),
			Self::LdAddr5(arg0) => write!(f, "LD   ${:04X}, A", arg0),
			Self::Jr(arg0) => write!(f, "JR   ${:04X}", arg0),
			Self::Sub(arg0) => write!(f, "SUB  {:?}", arg0),
			Self::Push(arg0) => write!(f, "PUSH {:?}", arg0),
			Self::Rl(arg0) => write!(f, "RL   {:?}", arg0),
			Self::Rla => write!(f, "RLA"),
			Self::Pop(arg0) => write!(f, "POP  {:?}", arg0),
			Self::Ret => write!(f, "RET"),
			// Self::Adc(arg0) => f.debug_tuple("Adc").field(arg0).finish(),
			Self::Adc8(arg0) => write!(f, "ADC  A, ${:02X}", arg0),
			Self::Cp(arg0) => write!(f, "CP   {:?}", arg0),
			Self::Add(arg0) => write!(f, "ADD  {:?}", arg0),
		}
	}
}

enum Prefix {
	None,
	CB,
}

#[derive(Debug, Clone, Copy)]
enum Reg {
	// 16bit
	BC,
	DE,
	HL,
	SP,
	AF,
	// 8 bit
	A,
	B,
	C,
	D,
	E,
	H,
	L,
}

#[derive(Debug, Clone, Copy)]
enum Flag {
	NZ,
	Z,
	NC,
	C,
}
