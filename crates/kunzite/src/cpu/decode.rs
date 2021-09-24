use crate::{cpu::instruction::Instruction, memory::Memory};

use super::{
	instruction::{Flag, Register16, Register8},
	Cpu,
};

const INSTRUCTION_TIMINGS: [i32; 256] = [
	4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4, 4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4,
	8, 4, 8, 12, 8, 8, 4, 4, 8, 4, 8, 8, 8, 8, 4, 4, 8, 4, 8, 12, 8, 8, 12, 12, 12, 4, 8, 8, 8, 8,
	4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
	4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 8, 8, 8, 8, 8, 8, 4, 8, 4, 4, 4, 4,
	4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
	4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
	4, 4, 8, 4, 8, 12, 12, 16, 12, 16, 8, 16, 8, 16, 12, 4, 12, 24, 8, 16, 8, 12, 12, 0, 12, 16, 8,
	16, 8, 16, 12, 0, 12, 0, 8, 16, 12, 12, 8, 0, 0, 16, 8, 16, 16, 4, 16, 0, 0, 0, 8, 16, 12, 12,
	8, 4, 0, 16, 8, 16, 12, 8, 16, 4, 0, 0, 8, 16,
];

const CB_INSTRUCTION_TIMINGS: [i32; 256] = [
	8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8,
	16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8,
	8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8,
	8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8,
	8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8,
	16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8,
	8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8,
	8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
	8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
];

impl Cpu {
	pub(super) fn fetch(&mut self, memory: &mut Memory) -> u8 {
		let opcode = self.get_n(memory);
		self.instruction_cycle += INSTRUCTION_TIMINGS[opcode as usize];
		opcode
	}

	fn get_n(&mut self, memory: &Memory) -> u8 {
		let byte = memory.read_byte(self.registers.pc);
		self.registers.pc += 1;

		byte
	}

	fn get_d(&mut self, memory: &Memory) -> i8 {
		self.get_n(memory) as i8
	}

	fn get_nn(&mut self, memory: &Memory) -> u16 {
		let word = memory.read_word(self.registers.pc);
		self.registers.pc += 2;

		word
	}

	pub fn decode(&self, pc: u16, memory: &Memory) -> String {
		let mut pc = pc;
		let mut opcode = memory.read_byte(pc);

		let inst = if opcode == 0xCB {
			pc += 1;
			opcode = memory.read_byte(pc);
			self.parse_normal_inst(DecodeInfo::new(memory, opcode, pc))
		} else {
			self.parse_cb_inst(DecodeInfo::new(memory, opcode, pc))
		};

		format!("{:?}", inst)
	}
}

#[derive(Debug)]
struct DecodeInfo {
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
	pub fn new(memory: &Memory, opcode: u8, pc: u16) -> Self {
		Self {
			x: (opcode >> 6) & 0x3,
			y: (opcode >> 3) & 0x7,
			z: opcode & 0x7,
			p: ((opcode >> 3) & 0x7) >> 1,
			q: ((opcode >> 3) & 0x7) % 2,
			d: memory.read_byte(pc + 1) as i8,
			n: memory.read_byte(pc + 1),
			nn: memory.read_word(pc + 1),
		}
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
					_ => Instruction::Mov8(R[y as usize], R[z as usize]),
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
					6 => Instruction::LoadCA,
					7 => Instruction::LoadAFromAddress(nn),
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
						1..=3 => panic!("removed\n{:#?}", info),
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
