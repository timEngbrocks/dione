use crate::{jvm::instructions::BranchKind, util::{sized_array::SizedArray, stack::Stack}};

use super::{execution_context::ExecutionContext, instructions::Instruction, types::Types, frame::Frame, object_manager::ObjectManager, descriptor::parse_method_descriptor};

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

	pub fn start(&mut self) {
		self.run("Main", "main", "([Ljava/lang/String;)V");
	}

	pub fn run(&mut self, class_name: &str, method_name: &str, descriptor: &str) {
		let execution_context = self.setup_method_execution(class_name, method_name, descriptor);
		let mut global_exception: Option<()> = None;
		self.current = Some(execution_context);
		loop {
			if let Some(_) = self.current  {
				let execution_context = self.current.as_mut().unwrap();

				if let Some(exception) = global_exception {
					if !execution_context.instruction_stream.try_handle(&exception) {
						// Unwind call stack
						if self.call_stack.is_empty() {
							break;
						}
						self.current = self.call_stack.pop();
						continue;
					}
					global_exception = None;
					continue;
				}

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
				global_exception = result.exception;
			} else {
				break;
			}
		}
	}

	fn setup_method_execution(&mut self, class_name: &str, method_name: &str, descriptor: &str) -> ExecutionContext {
		if let Some((object, method)) = ObjectManager::get(class_name).get_method(method_name, descriptor) {
			let return_type = match parse_method_descriptor(&method.descriptor) {
				Some((_, return_type)) => return_type,
				None => panic!("Invalid method descriptor: {}", method.descriptor),
			};

			let frame = Frame::new(
				SizedArray::<Types>::new(method.max_locals),
				Stack::<Types>::new(method.max_stack),
				&object.class_file,
				method.name.clone(),
				return_type,
			);
	
			let instruction_stream = method.instruction_stream.clone();
			
			ExecutionContext {
				frame,
				instruction_stream
			}
		} else {
			panic!("Method not found: {}{} on {}", method_name, descriptor, class_name);
		}
	}
}