use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        descriptor::{
            field_descriptor_is_array, field_descriptor_is_object, parse_field_descriptor,
        },
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        object_manager::ObjectManager,
        runtime_constant_pool::{RuntimeConstantPool, RuntimeConstants},
        types::{
            boolean::{Boolean, BooleanValue},
            int::Int,
            Types, Value,
        },
    },
    opcodes,
    util::heap::ReferencePtr,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PUTSTATIC {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for PUTSTATIC {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::PUTSTATIC));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        PUTSTATIC {
            indexbyte1,
            indexbyte2,
        }
    }

    // FIXME: final fields
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let index = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;
        let field_ref = match execution_context.runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefFieldOfClassOrInterface(value) => value,
            _ => panic!("Expected SymRefFieldOfClassOrInterface"),
        };
        let object = ObjectManager::get(&field_ref.class_ref.name);
        let field = match object.get_static_field(&field_ref.name, &field_ref.descriptor) {
            Some(value) => value,
            None => panic!(
                "Could not find field: {}{}",
                field_ref.name, field_ref.descriptor
            ),
        };
        let parsed_descriptor = match parse_field_descriptor(&field_ref.descriptor) {
            Some(value) => value,
            None => panic!("Invalid field descriptor: {}", &field_ref.descriptor),
        };
        let expected_value_type = match parsed_descriptor {
            Types::Boolean(_)
            | Types::Byte(_)
            | Types::Char(_)
            | Types::Short(_)
            | Types::Int(_) => Types::Int(Int::new()),
            _ => parsed_descriptor.clone(),
        };
        let value = execution_context.stack.pop();
        if field_descriptor_is_object(&field_ref.descriptor) {
            match &value {
                Types::Reference(reference) => {
                    if let ReferencePtr::Array(_) = reference.get() {
                        panic!("Expected ReferencePtr::Class/Interface/Null");
                    }
                }
                _ => panic!("Expected Reference"),
            };
        } else if field_descriptor_is_array(&field_ref.descriptor) {
            match &value {
                Types::Reference(reference) => match reference.get() {
                    ReferencePtr::Array(_) => (),
                    _ => panic!("Expected ReferencePtr::Array/Null"),
                },
                _ => panic!("Expected Reference"),
            };
        } else {
            expected_value_type.assert_matches_type(&value);
        }

        if let Types::Boolean(_) = parsed_descriptor {
            let x = match &value {
                Types::Int(x) => x.get(),
                _ => panic!("Expected Int"),
            };
            field.set_value(Some(Types::Boolean(Boolean::from_value(
                BooleanValue::from_value(x & 1),
            ))));
        } else {
            field.set_value(Some(value));
        }

        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        let index = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;
        let field_ref = match runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefFieldOfClassOrInterface(value) => value,
            _ => panic!("Expected SymRefFieldOfClassOrInterface"),
        };
        format!(
            "putstatic {} {} {}",
            field_ref.class_ref.name, field_ref.name, field_ref.descriptor
        )
    }
}
