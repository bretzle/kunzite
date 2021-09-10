pub fn upper(val: u16) -> u8 {
	(val >> 8) as u8
}

pub fn lower(val: u16) -> u8 {
	(val & 0xFF) as u8
}
