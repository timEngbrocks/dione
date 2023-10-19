pub struct ListNode {
	pub size: usize,
	pub next: Option<&'static mut ListNode>,
}

impl ListNode {
	pub const fn new(size: usize) -> Self {
		ListNode {
			size,
			next: None,
		}
	}

	pub fn start(&self) -> usize {
		self as *const Self as usize
	}

	pub fn end(&self) -> usize {
		self.start() + self.size
	}
}