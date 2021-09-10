use super::{
	instruction::{Flag, Instruction, Register16, Register8},
	Cpu,
};
use std::fmt::Debug;

struct DecodeInfo {
	pc: u16,
	x: u8,
	y: u8,
	z: u8,
	p: u8,
	q: u8,
	d: i8,
	n: u8,
	nn: u16,
}

impl DecodeInfo {
	#[allow(clippy::many_single_char_names)]
	pub fn new(cpu: &Cpu, opcode: u8) -> Self {
		let rom = cpu.rom.as_ref().unwrap();
		let x = (opcode >> 6) & 0x3;
		let y = (opcode >> 3) & 0x7;
		let z = opcode & 0x7;
		let p = y >> 1;
		let q = y % 2;

		let d = rom[cpu.pc as usize + 1] as i8;
		let n = rom[cpu.pc as usize + 1];
		let nn = if cpu.pc as usize + 2 < rom.len() {
			((rom[cpu.pc as usize + 2] as u16) << 8) | n as u16
		} else {
			0
		};

		Self {
			pc: cpu.pc,
			x,
			y,
			z,
			p,
			q,
			d,
			n,
			nn,
		}
	}

	fn opcode(&self) -> u8 {
		(self.x << 6) | (self.y << 3) | self.z
	}
}

impl Debug for DecodeInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "pc     = {:04X}\nopcode = {:#04X}\nx      = {:4}\ny      = {:4}\nz      = \
			{:4}\np      = {:4}\nq      = {:4}\nd      = {:4}\nn      = {:04X}\nnn     = \
			{:04X}\n-------------", self.pc, self.opcode(), self.x, self.y, self.z, self.p, self.q, self.d, self.n, self.nn)
	}
}

const CC: [Flag; 4] = [Flag::NZ, Flag::Z, Flag::NC, Flag::C];
const RP: [Register16; 4] = [
	Register16::BC,
	Register16::DE,
	Register16::HL,
	Register16::SP,
];
const RP2: [Register16; 4] = [
	Register16::BC,
	Register16::DE,
	Register16::HL,
	Register16::AF,
];
const R: [Register8; 8] = [
	Register8::B,
	Register8::C,
	Register8::D,
	Register8::E,
	Register8::H,
	Register8::L,
	Register8::DerefHL,
	Register8::A,
];

impl Cpu {
	/// Decodes a slice of a rom
	/// effectively dissasembles a program
	///
	/// Will panic if an invalid opcode is read
	///
	/// TODO: have parsing return an option to avoid panics
	pub fn try_decode_all(data: &[u8]) -> Vec<(u16, Instruction)> {
		let mut instructions = vec![];
		let mut this = Self::default();

		let _ = this.rom.insert(data.to_vec());

		while let Some(inst) = this.parse_instruction() {
			this.pc += inst.1.size();
			instructions.push(inst);
		}

		instructions
	}

	pub(super) fn parse_instruction(&self) -> Option<(u16, Instruction)> {
		let ret_pc = self.pc;
		let rom = self.rom.as_ref().unwrap();

		if let Some(&opcode) = rom.get(self.pc as usize) {
			let info = DecodeInfo::new(self, opcode);

			#[cfg(feature = "debug_opcode")]
			println!("{:?}", info);

			let inst = match opcode {
				0xCB => self.parse_cb_inst(DecodeInfo::new(self, rom[self.pc as usize + 1])),
				_ => self.parse_normal_inst(info),
			};

			Some((ret_pc, inst))
		} else {
			None
		}
	}

	fn parse_normal_inst(&self, info: DecodeInfo) -> Instruction {
		let DecodeInfo {
			x,
			y,
			z,
			p,
			q,
			d,
			n,
			nn,
			..
		} = info;

		match x {
			0 => match z {
				0 => match y {
					0 => Instruction::Nop,
					1 => Instruction::StoreImm16AddrSp(nn),
					2 => Instruction::Stop,
					3 => Instruction::Jr(None, d),
					4..7 => Instruction::Jr(Some(CC[y as usize - 4]), d),
					_ => unreachable!(),
				},
				1 => match q {
					0 => Instruction::StoreImm16(RP[p as usize], nn),
					1 => Instruction::AddHl(RP[p as usize]),
					_ => unreachable!(),
				},
				2 => match q {
					0 => match p {
						0 => Instruction::StoreATo16(Register16::BC),
						1 => Instruction::StoreATo16(Register16::DE),
						2 => Instruction::StoreAToHlAddr(true),
						3 => Instruction::StoreAToHlAddr(false),
						_ => unreachable!(),
					},
					1 => match p {
						0 => Instruction::LoadAFromReg16Addr(Register16::BC),
						1 => Instruction::LoadAFromReg16Addr(Register16::DE),
						2 => Instruction::LoadAFromHlAddr(true),
						3 => Instruction::LoadAFromHlAddr(false),
						_ => unreachable!(),
					},
					_ => unreachable!(),
				},
				3 => match q {
					0 => Instruction::Inc16(RP[p as usize]),
					1 => Instruction::Dec16(RP[p as usize]),
					_ => unreachable!(),
				},
				4 => Instruction::Inc8(R[y as usize]),
				5 => Instruction::Dec8(R[y as usize]),
				6 => Instruction::StoreImm8(R[y as usize], n),
				7 => match y {
					0 => Instruction::Rlca,
					1 => Instruction::Rrca,
					2 => Instruction::Rla,
					3 => Instruction::Rra,
					4 => Instruction::Daa,
					5 => Instruction::Cpl,
					6 => Instruction::Scf,
					7 => Instruction::Ccf,
					_ => unreachable!(),
				},
				_ => unreachable!(),
			},
			1 => match z {
				6 => match y {
					6 => Instruction::Halt,
					4 => Instruction::Mov8(Register8::H, Register8::DerefHL),
					_ => unreachable!(),
				},
				_ => Instruction::Mov8(R[y as usize], R[z as usize]),
			},
			2 => match y {
				0 => Instruction::Add(R[z as usize]),
				1 => Instruction::Adc(R[z as usize]),
				2 => Instruction::Sub(R[z as usize]),
				3 => Instruction::Sbc(R[z as usize]),
				4 => Instruction::And(R[z as usize]),
				5 => Instruction::Xor(R[z as usize]),
				6 => Instruction::Or(R[z as usize]),
				7 => Instruction::Cp(R[z as usize]),
				_ => unreachable!(),
			},
			3 => match z {
				0 => match y {
					0..=3 => Instruction::Ret(Some(CC[y as usize])),
					4 => Instruction::StoreHA(n),
					5 => Instruction::AddSp8(d),
					6 => Instruction::LoadHA(n),
					7 => Instruction::LdHlSp8(d),
					_ => unreachable!(),
				},
				1 => match q {
					0 => Instruction::Pop(RP2[p as usize]),
					1 => match p {
						0 => Instruction::Ret(None),
						1 => Instruction::Reti,
						2 => Instruction::JpHl,
						3 => Instruction::LdSpHl,
						_ => unreachable!(),
					},
					_ => unreachable!(),
				},
				2 => match y {
					0..3 => Instruction::Jp(Some(CC[y as usize]), nn),
					4 => Instruction::StoreCA,
					5 => Instruction::StoreAAtAddress(nn),
					6 => panic!("{:?}", info),
					7 => panic!("{:?}", info),
					_ => unreachable!(),
				},
				3 => match y {
					0 => Instruction::Jp(None, nn),
					1 => panic!("CB prefix"),
					2 => panic!("removed"),
					3 => panic!("removed"),
					4 => panic!("removed"),
					5 => panic!("removed"),
					6 => Instruction::Di,
					7 => Instruction::Ei,
					_ => unreachable!(),
				},
				4 => match y {
					0..=3 => Instruction::Call(Some(CC[y as usize]), nn),
					4..=7 => panic!("removed"),
					_ => unreachable!(),
				},
				5 => match q {
					0 => Instruction::Push(RP2[p as usize]),
					1 => match p {
						0 => Instruction::Call(None, nn),
						1..=3 => panic!("removed"),
						_ => unreachable!(),
					},
					_ => unreachable!(),
				},
				6 => match y {
					0 => Instruction::Add8(n),
					1 => Instruction::Adc8(n),
					2 => Instruction::Sub8(n),
					3 => Instruction::Sbc8(n),
					4 => Instruction::And8(n),
					5 => Instruction::Xor8(n),
					6 => Instruction::Or8(n),
					7 => Instruction::Cp8(n),
					_ => unreachable!(),
				},
				7 => Instruction::Rst(y * 8),
				_ => unreachable!(),
			},
			_ => unreachable!(),
		}
	}

	const fn parse_cb_inst(&self, info: DecodeInfo) -> Instruction {
		let DecodeInfo { x, y, z, .. } = info;

		let reg = R[z as usize];

		match x {
			0 => match y {
				0 => Instruction::Rlc(reg),
				1 => Instruction::Rrc(reg),
				2 => Instruction::Rl(reg),
				3 => Instruction::Rr(reg),
				4 => Instruction::Sla(reg),
				5 => Instruction::Sra(reg),
				6 => Instruction::Swap(reg),
				7 => Instruction::Srl(reg),
				_ => unreachable!(),
			},
			1 => Instruction::Bit(y, reg),
			2 => Instruction::Res(y, reg),
			3 => Instruction::Set(y, reg),
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn compare(instructions: Vec<Instruction>, asm: &str) {
		asm.lines()
			.filter(|line| {
				let line = line.trim();
				!line.starts_with(';') && line.len() > 1
			})
			.map(|s| s.split(';').next().unwrap())
			.enumerate()
			.for_each(|(idx, line)| {
				let calc = format!("{:?}", instructions[idx])
					.to_lowercase()
					.replace(' ', "");
				let real = line.trim().to_lowercase().replace(' ', "");

				assert_eq!(calc.trim(), real)
			});
	}

	#[test]
	fn test_decode_bootloader() {
		const BOOTLOADER: &[u8; 256] = include_bytes!("../../../../roms/bootloader.gb");
		const BOOTLOADER_ASM: &str = include_str!("../../../../roms/disassembly/bootloader.asm");

		let mut rom = BOOTLOADER.to_vec();
		rom.iter_mut()
			.enumerate()
			.filter(|(idx, _)| (0xA8..0xE0).contains(idx))
			.for_each(|(_, val)| *val = 0);

		let instructions = Cpu::try_decode_all(&rom)
			.into_iter()
			.map(|(_, inst)| inst)
			.collect();

		compare(instructions, BOOTLOADER_ASM);
	}
}
