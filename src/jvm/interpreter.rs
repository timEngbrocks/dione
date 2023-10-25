use crate::jvm::instructions::BranchKind;

use super::{execution_context::ExecutionContext, instructions::Instruction};

pub struct Interpreter {
	call_stack: Vec<ExecutionContext>,
	current: Option<ExecutionContext>,
}

impl Interpreter {
	pub fn new() -> Self {
		Interpreter {
			call_stack: Vec::new(),
			current: None,
		}
	}

	pub fn run(&mut self, object: &u8, method_name: &str) {
		let execution_context = self.setup_method_execution(object, method_name);
		self.current = Some(execution_context);
		loop {
			if let Some(_) = self.current  {
				let execution_context = self.current.as_mut().unwrap();
				if !execution_context.instruction_stream.has_next() {
					if self.call_stack.is_empty() {
						break;
					}
					let return_value = execution_context.frame.get_return_value().clone();
					self.current = self.call_stack.pop();
					self.current.as_mut().unwrap().frame.set_return_from_called_method(return_value);
					break;
				}
				let instruction = execution_context.instruction_stream.next();
				let result = instruction.execute(&mut execution_context.frame);
				// TODO: handle calls, exceptions, etc.
				if let Some(new_execution_context) = result.call {
					self.call_stack.push(self.current.clone().unwrap());
					self.current = Some(new_execution_context);
					break;
				}
				if let Some(branch) = result.branch {
					match branch {
						BranchKind::Absolute(value) => {
							execution_context.instruction_stream.absolute_jump(value as usize);
						},
						BranchKind::Relative(value) => {
							execution_context.instruction_stream.relative_jump(value as usize);
						},
					}
					break;
				}
			} else {
				break;
			}
		}
	}

	fn setup_method_execution(&mut self, _: &u8, _: &str) -> ExecutionContext {
		unimplemented!("Interpreter::setup_method_execution")
	}
}