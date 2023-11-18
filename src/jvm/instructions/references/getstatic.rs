use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        object_manager::ObjectManager,
        runtime_constant_pool::{RuntimeConstantPool, RuntimeConstants},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GETSTATIC {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for GETSTATIC {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::GETSTATIC));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        GETSTATIC {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
        let (object_name, name, descriptor) = execution_context.runtime_constant_pool().map(
            index,
            |constant| -> (String, String, String) {
                match constant {
                    RuntimeConstants::SymRefFieldOfClassOrInterface(field_ref) => (
                        field_ref.class_ref().name().clone(),
                        field_ref.name().clone(),
                        field_ref.descriptor().clone(),
                    ),
                    _ => panic!("Expected SymRefFieldOfClassOrInterface"),
                }
            },
        );
        // FIXME: initialize class, if not already initialized
        let object = ObjectManager::get(&object_name);
        let value = match object.get_static_field(&name, &descriptor) {
            Some(value) => match value.get_value() {
                Some(x) => x,
                None => panic!(
                    "Could not get value of static field {} on {}",
                    name, object_name
                ),
            },
            None => panic!("Could not find static field {} on {}", name, object_name),
        };
        execution_context.stack().push(value.clone());
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        let index = (self.indexbyte1 as u16) << 8 | self.indexbyte2 as u16;
        let field = match runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefFieldOfClassOrInterface(field) => field,
            _ => panic!("Expected SymRefFieldOfClassOrInterface"),
        };
        format!(
            "getstatic {}.{}:{}",
            field.class_ref().name(),
            field.name(),
            field.descriptor()
        )
    }
}
