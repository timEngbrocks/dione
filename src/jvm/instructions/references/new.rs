use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        object_manager::ObjectManager,
        runtime_constant_pool::{RuntimeConstantPool, RuntimeConstants},
        types::Types,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NEW {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for NEW {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::NEW));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        NEW {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let index = ((self.indexbyte1 as u16) << 8) | (self.indexbyte2 as u16);
        let class_ref = match execution_context.runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefClassOrInterface(class) => class,
            _ => panic!("Expected SymRefClassOrInterface"),
        };
        let reference = ObjectManager::instantiate(&class_ref.name);
        execution_context.stack.push(Types::Reference(reference));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("new: {}, {}", self.indexbyte1, self.indexbyte2)
    }
}
