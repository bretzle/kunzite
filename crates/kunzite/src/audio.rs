// Sound Channel 1 - Tone & Sweep
// Sound Channel 2 - Tone
// Sound Channel 3 - Wave Output
// Sound Channel 4 - Noise
#[derive(Default)]
pub struct Audio {
	/// Channel 1 Sweep register (R/W) [FF10]
	chan1_sweep: u8,
	/// Channel 1 Sound length/Wave pattern duty (R/W) [FF11]
	chan1_wave_duty: u8,
	/// Channel 1 Volume Envelope (R/W) [FF12]
	chan1_volume: u8,
	/// Channel 1 Frequency lo (Write Only) [FF13]
	chan1_freq_lo: u8,
	/// Channel 1 Frequency hi (R/W) [FF14]
	chan1_freq_hi: u8,

	/// Channel 2 Sound Length/Wave Pattern Duty (R/W) [FF16]
	chan2_wave_duty: u8,
	/// Channel 2 Volume Envelope (R/W) [FF17]
	chan2_volume: u8,
	/// Channel 2 Frequency lo data (W) [FF18]
	chan2_freq_lo: u8,
	/// Channel 2 Frequency hi data (R/W) [FF19]
	chan2_freq_hi: u8,

	/// Channel 3 Sound on/off (R/W) [FF1A]
	chan3_enable: u8,
	/// Channel 3 Sound Length (W) [FF1B]
	chan3_sound_length: u8,
	/// Channel 3 Select output level (R/W) [FF1C]
	chan3_select_output: u8,
	/// Channel 3 Frequency’s lower data (W) [FF1D]
	chan3_freq_lo: u8,
	/// Channel 3 Frequency’s higher data (R/W) [FF1E]
	chan3_freq_hi: u8,
	/// FF30-FF3F - Wave Pattern RAM
	chan3_wave_ram: [u8; 0x10],

	/// Channel 4 Sound Length (W) [FF20]
	chan4_sound_length: u8,
	/// Channel 4 Volume Envelope (R/W) [FF21]
	chan4_volume: u8,
	/// Channel 4 Polynomial Counter (R/W) [FF22]
	chan4_poly_counter: u8,
	/// Channel 4 Counter/consecutive; Inital (R/W) [FF23]
	chan4_linear_counter: u8,

	/// Channel control / ON-OFF / Volume (R/W) [FF24]
	channel_control: u8,
	/// Selection of Sound output terminal (R/W) [FF25]
	output_select: u8,
	/// Sound on/off [FF26]
	enable: u8,
}

impl Audio {
	pub fn new() -> Self {
		Self::default()
	}
}

impl Audio {
	pub fn write(&mut self, addr: usize, val: u8) {
		// TODO: some of these shouldn't be written to
		match addr {
			0xFF10 => self.chan1_sweep = val,
			0xFF11 => self.chan1_wave_duty = val,
			0xFF12 => self.chan1_volume = val,
			0xFF13 => self.chan1_freq_lo = val,
			0xFF14 => self.chan1_freq_hi = val,
			0xFF15 => (), // TODO: what is this???
			0xFF16 => self.chan2_wave_duty = val,
			0xFF17 => self.chan2_volume = val,
			0xFF18 => self.chan2_freq_lo = val,
			0xFF19 => self.chan2_freq_hi = val,
			0xFF1A => self.chan3_enable = val,
			0xFF1B => self.chan3_sound_length = val,
			0xFF1C => self.chan3_select_output = val,
			0xFF1D => self.chan3_freq_lo = val,
			0xFF1E => self.chan3_freq_hi = val,
			0xFF1F => (), // what is this???
			0xFF20 => self.chan4_sound_length = val,
			0xFF21 => self.chan4_volume = val,
			0xFF22 => self.chan4_poly_counter = val,
			0xFF23 => self.chan4_linear_counter = val,
			0xFF24 => self.channel_control = val,
			0xFF25 => self.output_select = val,
			0xFF26 => self.enable = val,
			0xFF27..0xFF30 => (), // what is this???
			0xFF30..0xFF40 => self.chan3_wave_ram[addr & 0xF] = val,
			_ => unreachable!("Unexpected address: 0x{:04x}", addr),
		}
	}

	pub fn read(&self, addr: usize) -> u8 {
		match addr {
			0xFF10 => self.chan1_sweep,
			0xFF11 => self.chan1_wave_duty,
			0xFF12 => self.chan1_volume,
			0xFF13 => self.chan1_freq_lo,
			0xFF14 => self.chan1_freq_hi,
			0xFF15 => 0, // TODO: what is this???
			0xFF16 => self.chan2_wave_duty,
			0xFF17 => self.chan2_volume,
			0xFF18 => self.chan2_freq_lo,
			0xFF19 => self.chan2_freq_hi,
			0xFF1A => self.chan3_enable,
			0xFF1B => self.chan3_sound_length,
			0xFF1C => self.chan3_select_output,
			0xFF1D => self.chan3_freq_lo,
			0xFF1E => self.chan3_freq_hi,
			0xFF1F => 0, // what is this???
			0xFF20 => self.chan4_sound_length,
			0xFF21 => self.chan4_volume,
			0xFF22 => self.chan4_poly_counter,
			0xFF23 => self.chan4_linear_counter,
			0xFF24 => self.channel_control,
			0xFF25 => self.output_select,
			0xFF26 => self.enable,
			0xFF27..0xFF30 => 0, // what is this???
			0xFF30..0xFF40 => self.chan3_wave_ram[addr & 0xF],
			_ => unreachable!("Unexpected address: 0x{:04x}", addr),
		}
	}

	pub fn update(&mut self, _tick: u8) {}
}
