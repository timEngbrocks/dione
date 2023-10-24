use super::{instructions::InstructionStream, frame::Frame};

#[derive(Clone)]
pub struct ExecutionContext {
	pub frame: Frame,
	pub instruction_stream: InstructionStream,
}

impl ExecutionContext {
	pub fn new(frame: Frame, instruction_stream: InstructionStream) -> Self {
		ExecutionContext {
			frame,
			instruction_stream,
		}
	}
}