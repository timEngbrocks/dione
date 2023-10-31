#[derive(Clone)]
pub struct ExceptionHandler {
	pub start_pc: u16,
	pub end_pc: u16,
	pub handler_pc: u16,
	pub catch_type: u16,
}

#[derive(Clone)]
pub struct ExceptionHandlerTable {
	exception_handler: Vec<ExceptionHandler>,
}

impl ExceptionHandlerTable {
	pub fn new() -> Self {
		ExceptionHandlerTable {
			exception_handler: Vec::new(),
		}
	}

	pub fn add(&mut self, exception_handler: ExceptionHandler) {
		self.exception_handler.push(exception_handler);
	}

	pub fn try_handle(&mut self, _: &()) -> Option<ExceptionHandler> {
		// 1. Find handlers responsible for start_pc -> end_pc code block
		// 2. Find handler responsible for catch_type
		None
	}
}