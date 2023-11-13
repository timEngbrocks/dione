use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::{RuntimeConstants, RuntimeConstantPool},
        types::{Types, Value},
    },
    opcodes,
    util::heap::ReferencePtr,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GETFIELD {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for GETFIELD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::GETFIELD));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        GETFIELD {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let index = (self.indexbyte1 as U2) << 8 | self.indexbyte2 as U2;
        let field_ref = match execution_context.runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefFieldOfClassOrInterface(field_ref) => field_ref,
            _ => panic!("Expected SymRefFieldOfClassOrInterface"),
        };
        let object_ref = match execution_context.stack.pop() {
            Types::Reference(reference) => match reference.get() {
                ReferencePtr::Class(object_ref) => object_ref,
                _ => panic!("Expected object_ref"),
            },
            _ => panic!("Expected Reference"),
        };
        let object = object_ref.borrow_mut();
        // FIXME: Check if field_ref.class_ref is a superclass of object
        // assert_eq!(object.name, field_ref.class_ref.name);
        let value = match object.get_field(&field_ref.name, &field_ref.descriptor) {
            Some(field) => match field.get_value() {
                Some(value) => value,
                None => panic!("Expected value"),
            },
            None => panic!("Expected field"),
        };
        execution_context.stack.push(value.clone());
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        let index = (self.indexbyte1 as U2) << 8 | self.indexbyte2 as U2;
        let field_ref = match runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefFieldOfClassOrInterface(field_ref) => field_ref,
            _ => panic!("Expected SymRefFieldOfClassOrInterface"),
        };
        format!(
            "getfield {} {}",
            field_ref.name,
            field_ref.descriptor,
        )
    }
}
