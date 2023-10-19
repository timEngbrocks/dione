pub fn align_up(address: usize, align: usize) -> usize {
	(address + align - 1) & !(align - 1)
}