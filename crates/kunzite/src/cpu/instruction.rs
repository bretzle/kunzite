//! TODO: Document this

/// Instructions that the Gameboy can execute
///
/// Naming tends to be tof the form: `ActionDestSrc` when there is ambiguity
///
/// Eg. `Instruction::StoreImm16AddrSp` means that the `SP` register shoudl be stored at the address specified by the immediat 16 bit value
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
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
	Jr(Option<Flag>, i8),
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
	AddSp8(i8),
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
	StoreHA(u8), // 0xE0
	/// loads A from (u8)
	LoadHA(u8), // 0xF0
	/// stores A in (C)
	StoreCA,
	/// Loads A from (C)
	LoadCA,
	/// LD (a16), A
	StoreAAtAddress(u16), // 0xEA
	/// LD A, (a16)
	LoadAFromAddress(u16), // 0xFA

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

impl std::fmt::Debug for Instruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Nop => write!(f, "NOP"),
			Self::Stop => write!(f, "STOP"),
			Self::Halt => write!(f, "HALT"),
			Self::StoreImm16(arg0, arg1) => write!(f, "LD  {:?}, ${:04X}", arg0, arg1),
			Self::StoreImm8(arg0, arg1) => write!(f, "LD  {:?}, ${:02X}", arg0, arg1),
			Self::StoreAToHlAddr(arg0) => {
				let sign = if *arg0 { "+" } else { "-" };
				write!(f, "LD  (HL{}), A", sign)
			}
			Self::LoadAFromHlAddr(arg0) => f.debug_tuple("LoadAFromHlAddr").field(arg0).finish(),
			Self::StoreATo16(arg0) => f.debug_tuple("StoreATo16").field(arg0).finish(),
			Self::LoadAFromReg16Addr(arg0) => write!(f, "LD  A, ({:?})", arg0),
			Self::Mov8(arg0, arg1) => write!(f, "LD  {:?}, {:?}", arg0, arg1),
			Self::Jr(arg0, arg1) => match arg0 {
				Some(fl) => write!(f, "JR  {:?}, ${:02X}", fl, arg1),
				None => write!(f, "JR  ${:02X}", arg1),
			},
			Self::Jp(arg0, arg1) => f.debug_tuple("Jp").field(arg0).field(arg1).finish(),
			Self::Inc8(arg0) => write!(f, "INC {:?}", arg0),
			Self::Dec8(arg0) => write!(f, "DEC {:?}", arg0),
			Self::Inc16(arg0) => write!(f, "INC {:?}", arg0),
			Self::Dec16(arg0) => write!(f, "DEC {:?}", arg0),
			Self::Push(arg0) => write!(f, "PUSH {:?}", arg0),
			Self::Pop(arg0) => write!(f, "POP {:?}", arg0),
			Self::Add(arg0) => write!(f, "ADD {:?}", arg0),
			Self::Adc(arg0) => write!(f, "ADC {:?}", arg0),
			Self::Sub(arg0) => write!(f, "SUB {:?}", arg0),
			Self::Sbc(arg0) => write!(f, "SBC {:?}", arg0),
			Self::And(arg0) => write!(f, "AND {:?}", arg0),
			Self::Xor(arg0) => write!(f, "XOR {:?}", arg0),
			Self::Or(arg0) => write!(f, "OR  {:?}", arg0),
			Self::Cp(arg0) => write!(f, "CP  {:?}", arg0),
			Self::Add8(arg0) => write!(f, "ADD ${:02X}", arg0),
			Self::Adc8(arg0) => write!(f, "ADC ${:02X}", arg0),
			Self::Sub8(arg0) => write!(f, "SUB ${:02X}", arg0),
			Self::Sbc8(arg0) => write!(f, "SBC ${:02X}", arg0),
			Self::And8(arg0) => write!(f, "AND ${:02X}", arg0),
			Self::Xor8(arg0) => write!(f, "XOR ${:02X}", arg0),
			Self::Or8(arg0) => write!(f, "OR  ${:02X}", arg0),
			Self::Cp8(arg0) => write!(f, "CP  ${:02X}", arg0),
			Self::AddSp8(arg0) => f.debug_tuple("AddSp8").field(arg0).finish(),
			Self::Daa => write!(f, "DAA"),
			Self::Scf => write!(f, "SCF"),
			Self::Cpl => write!(f, "CPL"),
			Self::Ccf => write!(f, "CCF"),
			Self::Rlca => write!(f, "RLCA"),
			Self::Rla => write!(f, "RLA"),
			Self::Rrca => write!(f, "RRCA"),
			Self::Rra => write!(f, "RRA"),
			Self::StoreImm16AddrSp(arg0) => write!(f, "LD  ${:04X}, SP", arg0),
			Self::AddHl(arg0) => f.debug_tuple("AddHl").field(arg0).finish(),
			Self::Ret(arg0) => match arg0 {
				Some(fl) => write!(f, "RET {:?}", fl),
				None => write!(f, "RET"),
			},
			Self::Reti => write!(f, "RETI"),
			Self::Di => write!(f, "DI"),
			Self::Ei => write!(f, "EI"),
			Self::Call(arg0, arg1) => match arg0 {
				Some(fl) => write!(f, "CALL {:?}, ${:04X}", fl, arg1),
				None => write!(f, "CALL ${:04X}", arg1),
			},
			Self::JpHl => write!(f, "JpHl"),
			Self::Rst(arg0) => f.debug_tuple("Rst").field(arg0).finish(),
			Self::LdHlSp8(arg0) => f.debug_tuple("LdHlSp8").field(arg0).finish(),
			Self::LdSpHl => write!(f, "LdSpHl"),
			Self::StoreHA(arg0) => write!(f, "LD  ($FF00+${:02X}), A", arg0),
			Self::LoadHA(arg0) => write!(f, "LD  A, ($FF00+${:02X})", arg0),
			Self::StoreCA => write!(f, "LD  ($FF00+C), A"),
			Self::LoadCA => write!(f, "LoadCA"),
			Self::StoreAAtAddress(arg0) => write!(f, "LD  (${:04X}), A", arg0),
			Self::LoadAFromAddress(arg0) => f.debug_tuple("LoadAFromAddress").field(arg0).finish(),
			Self::Rlc(arg0) => write!(f, "RLC {:?}", arg0),
			Self::Rrc(arg0) => write!(f, "RRC {:?}", arg0),
			Self::Rr(arg0) => write!(f, "RR  {:?}", arg0),
			Self::Rl(arg0) => write!(f, "RL  {:?}", arg0),
			Self::Sla(arg0) => f.debug_tuple("Sla").field(arg0).finish(),
			Self::Sra(arg0) => f.debug_tuple("Sra").field(arg0).finish(),
			Self::Swap(arg0) => f.debug_tuple("Swap").field(arg0).finish(),
			Self::Srl(arg0) => f.debug_tuple("Srl").field(arg0).finish(),
			Self::Bit(arg0, arg1) => write!(f, "BIT {}, {:?}", arg0, arg1),
			Self::Res(arg0, arg1) => f.debug_tuple("Res").field(arg0).field(arg1).finish(),
			Self::Set(arg0, arg1) => f.debug_tuple("Set").field(arg0).field(arg1).finish(),
		}
	}
}

impl Instruction {
	/// the number of bytes the instruction takes up
	pub fn size(&self) -> u16 {
		match self {
			Instruction::Nop => 1,
			Instruction::Stop => 2,
			Instruction::Halt => 1,
			Instruction::StoreImm16(_, _) => 3,
			Instruction::StoreImm8(_, _) => 2,
			Instruction::StoreAToHlAddr(_) => 1,
			Instruction::LoadAFromHlAddr(_) => todo!(),
			Instruction::StoreATo16(_) => todo!(),
			Instruction::LoadAFromReg16Addr(_) => 1,
			Instruction::Mov8(_, _) => 1,
			Instruction::Jr(_, _) => 2,
			Instruction::Jp(_, _) => 3,
			Instruction::Inc8(_) => 1,
			Instruction::Dec8(_) => 1,
			Instruction::Inc16(_) => 1,
			Instruction::Dec16(_) => 1,
			Instruction::Push(_) => 1,
			Instruction::Pop(_) => 1,
			Instruction::Add(_) => 1,
			Instruction::Adc(_) => 1,
			Instruction::Sub(_) => 1,
			Instruction::Sbc(_) => 1,
			Instruction::And(_) => 1,
			Instruction::Xor(_) => 1,
			Instruction::Or(_) => 1,
			Instruction::Cp(_) => 1,
			Instruction::Add8(_) => 2,
			Instruction::Adc8(_) => 2,
			Instruction::Sub8(_) => 2,
			Instruction::Sbc8(_) => 2,
			Instruction::And8(_) => 2,
			Instruction::Xor8(_) => 2,
			Instruction::Or8(_) => 2,
			Instruction::Cp8(_) => 2,
			Instruction::AddSp8(_) => 2,
			Instruction::Daa => 1,
			Instruction::Scf => 1,
			Instruction::Cpl => 1,
			Instruction::Ccf => 1,
			Instruction::Rlca => 1,
			Instruction::Rla => 1,
			Instruction::Rrca => 1,
			Instruction::Rra => 1,
			Instruction::StoreImm16AddrSp(_) => 3,
			Instruction::AddHl(_) => todo!(),
			Instruction::Ret(_) => 1,
			Instruction::Reti => 1,
			Instruction::Di => 1,
			Instruction::Ei => 1,
			Instruction::Call(_, _) => 3,
			Instruction::JpHl => 1,
			Instruction::Rst(_) => 1,
			Instruction::LdHlSp8(_) => todo!(),
			Instruction::LdSpHl => todo!(),
			Instruction::StoreHA(_) => 2,
			Instruction::LoadHA(_) => 2,
			Instruction::StoreCA => 1,
			Instruction::LoadCA => todo!(),
			Instruction::StoreAAtAddress(_) => 3,
			Instruction::LoadAFromAddress(_) => 3,
			Instruction::Rlc(_) => 2,
			Instruction::Rrc(_) => 2,
			Instruction::Rr(_) => 2,
			Instruction::Rl(_) => 2,
			Instruction::Sla(_) => 2,
			Instruction::Sra(_) => 2,
			Instruction::Swap(_) => 2,
			Instruction::Srl(_) => 2,
			Instruction::Bit(_, _) => 2,
			Instruction::Res(_, _) => 2,
			Instruction::Set(_, _) => 2,
		}
	}
}

#[allow(missing_docs)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Register8 {
	A,
	B,
	C,
	D,
	E,
	H,
	L,
	DerefHL,
	F,
}

impl std::fmt::Debug for Register8 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::A => write!(f, "A"),
			Self::B => write!(f, "B"),
			Self::C => write!(f, "C"),
			Self::D => write!(f, "D"),
			Self::E => write!(f, "E"),
			Self::H => write!(f, "H"),
			Self::L => write!(f, "L"),
			Self::DerefHL => write!(f, "(HL)"),
			Self::F => unreachable!(),
		}
	}
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

impl Register16 {
	pub fn tear(&self) -> (Register8, Register8) {
		match self {
			Register16::BC => (Register8::B, Register8::C),
			Register16::DE => (Register8::D, Register8::E),
			Register16::HL => (Register8::H, Register8::L),
			Register16::AF => (Register8::A, Register8::F),
			Register16::SP => unreachable!(),
		}
	}
}

/// CPU flags
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Flag {
	/// Zero
	Z,
	/// Not Zero
	NZ,
	/// Subtract
	N,
	/// Half-carry
	H,
	/// Carry
	C,
	/// Not Carry
	NC,
}
