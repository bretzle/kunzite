pub fn upper(val: u16) -> u8 {
	(val >> 8) as u8
}

pub fn lower(val: u16) -> u8 {
	(val & 0xFF) as u8
}

pub fn slice_to_string(slice: &[u8]) -> String {
	slice
		.iter()
		.filter(|b| **b != 0)
		.map(|b| *b as char)
		.collect()
}
