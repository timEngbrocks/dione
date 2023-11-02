use crate::{jvm::{frame::Frame, types::{Value, Types, array::{Array, ReferenceArray, ReferenceArrayKind}, reference::Reference}, instructions::{Instruction, InstructionResult}, runtime_constant_pool::RuntimeConstants, object_manager::ObjectManager}, class_loader::parser::{Parser, U2, U1}, opcodes, util::heap::Heap};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ANEWARRAY {
	indexbyte1: U1,
	indexbyte2: U1,
}
impl Instruction for ANEWARRAY {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::ANEWARRAY));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		ANEWARRAY {
			indexbyte1,
			indexbyte2,
		}
	}

	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		let count = match execution_context.stack.pop() {
			Types::Int(value) => value.get() as usize,
			_ => panic!("Expected Int"),
		};
		let index = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;
		let class_ref = match execution_context.runtime_constant_pool.get(index) {
			RuntimeConstants::SymRefClassOrInterface(value) => value,
			_ => panic!("Expected SymRefClassOrInterface"),
		};
		
		let array = match class_ref.clone() {
			class_ref if ObjectManager::is_class(&class_ref.name) => {
				ReferenceArray::new(ReferenceArrayKind::Class(class_ref, Reference::new()), count)
			},
			class_ref if ObjectManager::is_interface(&class_ref.name) => {
				ReferenceArray::new(ReferenceArrayKind::Interface(class_ref, Reference::new()), count)
			},
			class_ref if ObjectManager::is_array_class(&class_ref.name) => {
				ReferenceArray::new(ReferenceArrayKind::Array(class_ref, Reference::new()), count)
			},
			_ => panic!("Invalid class reference"),
		};

		let reference = Heap::allocate_reference_array(array);
		execution_context.stack.push(Types::Reference(reference));
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		3
	}
}