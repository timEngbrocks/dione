use crate::{jvm::{types::Types, frame::Frame, instructions::{Instruction, InstructionResult}, runtime_constant_pool::{RuntimeConstants, sym_ref_method_of_class::SymRefMethodOfClass, sym_ref_method_of_interface::SymRefMethodOfInterface}, object_manager::ObjectManager, descriptor::parse_method_descriptor, execution_context::ExecutionContext}, class_loader::parser::{Parser, U2, U1}, opcodes, util::{sized_array::SizedArray, stack::Stack}};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKESPECIAL {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for INVOKESPECIAL {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::INVOKESPECIAL));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		INVOKESPECIAL {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		let index = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;
		let (object, method) = match execution_context.runtime_constant_pool.get(index) {
			RuntimeConstants::SymRefMethodOfClass(SymRefMethodOfClass { name, descriptor, class_ref }) => {
				let object = ObjectManager::get(class_ref.name.as_str());
				object.get_method(name.as_str(), descriptor.as_str()).unwrap()
			},
			RuntimeConstants::SymRefMethodOfInterface(SymRefMethodOfInterface { name, descriptor, class_ref }) => {
				let object = ObjectManager::get(class_ref.name.as_str());
				object.get_method(name.as_str(), descriptor.as_str()).unwrap()
			},
			_ => panic!("Expected SymRefMethodOfClass or SymRefMethodOfInterface"),
		};
		let parse_result = parse_method_descriptor(&method.descriptor);
		let (mut arg_types, return_type) = match parse_result {
			Some((arg_types, return_type)) => (arg_types, return_type),
			None => panic!("Invalid method descriptor: {}", method.descriptor),
		};
		arg_types = arg_types.iter().rev().map(|arg| {
			let actual_arg = execution_context.stack.pop();
			actual_arg.assert_matches_type(arg);
			actual_arg
		}).collect::<Vec<Types>>();
		let object_ref = match execution_context.stack.pop() {
			Types::Reference(object_ref) => object_ref,
			_ => panic!("INVOKESPECIAL: Expected Reference for object_ref"),
		};

		if method.is_synchronized() {
			unimplemented!("INVOKESPECIAL: synchronized")
		}

		if method.is_native() {
			unimplemented!("INVOKESPECIAL: native")
		}

		let mut local_variables = SizedArray::<Types>::new(method.max_locals);
		local_variables.set(0, Types::Reference(object_ref.clone()));
		for i in 0..arg_types.len() {
			local_variables.set((i + 1) as u16, arg_types[i].clone());
		}
		let stack = Stack::<Types>::new(method.max_stack);
		let frame = Frame::new(
			local_variables,
			stack,
			&object.class_file,
			method.name.clone(),
			return_type,
		);
		
		InstructionResult::call(ExecutionContext::new(frame, method.instruction_stream.clone()))
	}

	fn length(&self) -> U2 {
		3
	}
}