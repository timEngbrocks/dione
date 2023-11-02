use crate::{jvm::{frame::Frame, instructions::{Instruction, InstructionResult}, types::{Types, Value, array::{ReferenceArrayKind, ReferenceArray}, reference::Reference}, runtime_constant_pool::RuntimeConstants, object_manager::ObjectManager}, class_loader::parser::{Parser, U2, U1}, opcodes, util::heap::Heap};

use crate::jvm::types::array::Array;

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MULTIANEWARRAY {
	indexbyte1: U1,
	indexbyte2: U1,
	dimensions: U1,
}
impl Instruction for MULTIANEWARRAY {
	fn new(parser: &mut Parser) -> Self where Self: Sized {
		let opcode = parser.consume_u1();
		assert_eq!(opcode, opcodes!(Instructions::MULTIANEWARRAY));
		let indexbyte1 = parser.consume_u1();
		let indexbyte2 = parser.consume_u1();
		let dimensions = parser.consume_u1();
		MULTIANEWARRAY {
			indexbyte1,
			indexbyte2,
			dimensions,
		}
	}

	// FIXME: If array class check that dimensionality is >= count
	// FIXME: If count == 0 for any dimension => Deeper dimensions are not allocated
	fn execute(&mut self, execution_context: &mut Frame) -> InstructionResult {
		assert!(self.dimensions >= 1);
		let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
		let class_ref = match execution_context.runtime_constant_pool.get(index) {
			RuntimeConstants::SymRefClassOrInterface(value) => value,
			_ => panic!("Expected SymRefClassOrInterface"),
		};
		let count = match execution_context.stack.pop() {
			Types::Int(value) => value.get() as usize,
			_ => panic!("Expected Int"),
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
		let reference = (1..self.dimensions).fold(reference, |reference, _index| {
			let count = match execution_context.stack.pop() {
				Types::Int(value) => value.get() as usize,
				_ => panic!("Expected Int"),
			};
			let array = ReferenceArray::new_preinitialized(ReferenceArrayKind::Array(class_ref.clone(), reference), count);
			Heap::allocate_reference_array(array)
		});
		execution_context.stack.push(Types::Reference(reference));
		InstructionResult::empty()
	}

	fn length(&self) -> U2 {
		4
	}

	fn to_string(&self) -> String {
		format!("multianewarray: {}, {}, {}", self.indexbyte1, self.indexbyte2, self.dimensions)
	}
}