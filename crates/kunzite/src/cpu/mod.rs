//! The cpu

/// The cpu
pub struct Cpu {
	rom: Vec<u8>,
	pc: usize,
}

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
	pub fn new(cpu: &Cpu, opcode: u8) -> Self {
		let x = (opcode >> 6) & 0x3;
		let y = (opcode >> 3) & 0x7;
		let z = opcode & 0x7;
		let p = y >> 1;
		let q = y % 2;

		let d = cpu.rom[cpu.pc + 1] as i8;
		let n = cpu.rom[cpu.pc + 1];
		let nn = if cpu.pc + 2 < cpu.rom.len() {
			((cpu.rom[cpu.pc + 2] as u16) << 8) | n as u16
		} else {
			0
		};

		Self {
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
}

impl Cpu {
	/// Decodes a slice of a rom
	/// effectively dissasembles a program
	pub fn decode_all(data: &[u8]) -> Vec<(usize, Instruction)> {
		let mut instructions = vec![];
		let this = Self {
			rom: data.to_vec(),
			pc: 0,
		};

		while let Some(inst) = this.parse_instruction() {
			instructions.push(inst);
		}

		instructions
	}

	fn parse_instruction(&self) -> Option<(usize, Instruction)> {
		let ret_pc = self.pc;

		if let Some(&opcode) = self.rom.get(self.pc) {
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
				"pc     = {:04X}\nopcode = {:#04X}\nx      = {:4}\ny      = {:4}\nz      = \
				 {:4}\np      = {:4}\nq      = {:4}\nd      = {:4}\nn      = {:04X}\nnn     = \
				 {:04X}\n-------------",
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

			let inst = match x {
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
			};

			self.pc += inst.size();

			Some((ret_pc, inst))
		} else {
			None
		}
	}

	fn parse_normal_inst() -> Option<Instruction> {
		None
	}

	fn parse_cb_inst() -> Option<Instruction> {
		None
	}
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Register8 {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
	DerefHL,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Register16 {
	BC,
	DE,
	HL,
	AF,
	SP,
}

/// CPU flags
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Flag {
	/// Carry
	C,
	/// Zero
	Z,
	/// Not Carry
	NC,
	/// Not Zero
	NZ,
}

/// Instructions that the Gameboy can execute
///
/// Naming tends to be tof the form: `ActionDestSrc` when there is ambiguity
///
/// Eg. `Instruction::StoreImm16AddrSp` means that the `SP` register shoudl be stored at the address specified by the immediat 16 bit value
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Instruction {
	/// No operation.
	Nop,
	/// The Gameboy enters a very low-power STOP state, graphics will not continue to draw.
	Stop,
	/// The Gameboy enters a low-power HALT state.
	Halt,
	/// Store an immediate value into a 16 bit register.
	StoreImm16(Register16, u16),
	/// Store an immediate value into an 8 bit register.
	StoreImm8(Register8, u8),
	/// Store A at (HL) and increment or decrement HL; true means inc
	StoreAToHlAddr(bool),
	/// Load A from (HL) and increment or decrement HL; true means inc
	LoadAFromHlAddr(bool),
	/// Store A to the value pointed at by register 16 (must be BC or DE)
	StoreATo16(Register16),
	/// Loads A from value pointed at by register 16 (must be BC or DE)
	LoadAFromReg16Addr(Register16),
	/// Move the value of one register to another
	Mov8(Register8, Register8),
	/// Relative jump based on flag to offset
	Jr(Option<Flag>, u8),
	/// Jump based on flag to offset
	Jp(Option<Flag>, u16),
	/// Increment an 8 bit regsiter.
	Inc8(Register8),
	/// Decrement an 8 bit regsiter.
	Dec8(Register8),
	/// Increment a 16 bit regsiter.
	Inc16(Register16),
	/// Decrement a 16 bit regsiter.
	Dec16(Register16),
	/// Push the value in the given register onto the stack.
	Push(Register16),
	/// Pop a value off the stack and load it into the given register.
	Pop(Register16),
	/// Add the given regsiter to the A.
	Add(Register8),
	/// Add the given regsiter to the A with carry.
	Adc(Register8),
	/// Subtract the given regsiter from the A.
	Sub(Register8),
	/// Subtract the given regsiter from the A with carry.
	Sbc(Register8),
	/// Bitwise AND the given register with the A.
	And(Register8),
	/// Bitwise XOR the given register with the A.
	Xor(Register8),
	/// Bitwise OR the given register with the A.
	Or(Register8),
	/// Compare the value of the given register with the A and set flags.
	Cp(Register8),
	/// Add an immediate value to the A.
	Add8(u8),
	/// Add an immediate value to the A with carry.
	Adc8(u8),
	/// Subtract an immediate value from the A.
	Sub8(u8),
	/// Subtract an immediate value from the A with carry.
	Sbc8(u8),
	/// Bitwise AND an immediate value with the A.
	And8(u8),
	/// Bitwise XOR an immediate value with the A.
	Xor8(u8),
	/// Bitwise OR an immediate value with the A.
	Or8(u8),
	/// Compare the immediate value with the A and set flags.
	Cp8(u8),
	/// Add the immediate value to the Program Counter and load it into SP.
	/// TODO: check this explanation
	AddSp8(u8),
	/// Converts the value in A to its BCD form.
	/// TODO: double check this
	Daa,
	/// TODO: document this
	Scf,
	/// Bitwise negate the value in the A.
	Cpl,
	/// TODO: document this (inverse of SCF?)
	Ccf,
	/// Rotate A left.
	Rlca,
	/// Rotate A left through carry.
	Rla,
	/// Rotate A right.
	Rrca,
	/// Rotate A right through carry.
	Rra,
	/// Stores SP at pointer given by immediate 16.
	StoreImm16AddrSp(u16),
	/// Adds a value to HL.
	AddHl(Register16),
	/// Conditionally adjusts the program counter and updates the stack pointer.
	Ret(Option<Flag>),
	/// Non-conditional `Ret` that also enables interrupts.
	Reti,
	/// Disable interrupts.
	Di,
	/// Enable interrupts.
	Ei,
	/// Conditionally update push the program counter onto the stack and adjusts
	/// the program counter.
	Call(Option<Flag>, u16),
	/// Gets the value at memory address HL and jumps to it.
	JpHl,
	/// Contains eight possible values: between 0-8. Value should be multplied
	/// by 8 to determine the reset location.
	/// TODO: consider simplifying this
	Rst(u8),
	/// HL = SP + (PC + i8).
	/// TODO: double check behavior of relative parameters.
	LdHlSp8(i8),
	/// Load the value of HL into SP.
	LdSpHl,
	/// stores A in (u8)
	StoreHA(u8),
	/// loads A from (u8)
	LoadHA(u8),
	/// stores A in (C)
	StoreCA,
	/// Loads A from (C)
	LoadCA,
	/// LD (a16), A
	StoreAAtAddress(u16),
	/// LD A, (a16)
	LoadAFromAddress(u16),

	// 0xCB instructions
	/// Rotate register left.
	Rlc(Register8),
	/// Rotate register right.
	Rrc(Register8),
	/// Rotate register right through carry.
	Rr(Register8),
	/// Rotate register left through carry.
	Rl(Register8),
	/// Arithmetic left shift on given register.
	Sla(Register8),
	/// Arithmetic right shift on given register.
	Sra(Register8),
	/// Swap low and high nibble (4 bits).
	Swap(Register8),
	/// Logical Right shift on given register.
	Srl(Register8),
	/// Set flags based on the given bit in register.
	/// u8 is number between 0 and 7 (inclusive).
	Bit(u8, Register8),
	/// Reset the given bit in the register.
	/// u8 is number between 0 and 7 (inclusive)
	Res(u8, Register8),
	/// Set the given bit in the register.
	/// u8 is number between 0 and 7 (inclusive)
	Set(u8, Register8),
}
