use core::panic;

use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        descriptor::parse_method_descriptor,
        execution_context::ExecutionContext,
        frame::Frame,
        instructions::{Instruction, InstructionResult, InstructionStream},
        object_manager::ObjectManager,
        runtime_constant_pool::{RuntimeConstantPool, RuntimeConstants},
        types::Types,
    },
    opcodes,
    util::{sized_array::SizedArray, stack::Stack},
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKESTATIC {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for INVOKESTATIC {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::INVOKESTATIC));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        INVOKESTATIC {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let index = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;
        let (object_name, name, descriptor) =
            match execution_context.runtime_constant_pool().get(index) {
                RuntimeConstants::SymRefMethodOfClass(symref) => (
                    symref.class_ref().name().clone(),
                    symref.name().clone(),
                    symref.descriptor().clone(),
                ),
                RuntimeConstants::SymRefMethodOfInterface(symref) => (
                    symref.class_ref().name().clone(),
                    symref.name().clone(),
                    symref.descriptor().clone(),
                ),
                _ => panic!("Expected SymRefMethodOfClass or SymRefMethodOfInterface"),
            };
        let object = ObjectManager::get(&object_name);
        let (_, method) = object.get_method(&object_name, &descriptor).unwrap();
        let parse_result = parse_method_descriptor(method.descriptor());
        let (arg_types, return_type) = match parse_result {
            Some((arg_types, return_type)) => (arg_types, return_type),
            None => panic!("Invalid method descriptor: {}", method.descriptor()),
        };
        let mut actual_arg_types = Vec::with_capacity(arg_types.len());
        for arg in arg_types.iter().rev() {
            let actual_arg = execution_context.stack().pop();
            actual_arg.assert_matches_type(arg);
            actual_arg_types.push(actual_arg);
        }

        if !method.is_static() || method.is_abstract() {
            panic!("Expected static method")
        }

        ObjectManager::initialize_object(object.name());

        let (max_locals, max_stack) = if method.is_native() {
            (None, None)
        } else {
            (Some(*method.max_locals()), Some(*method.max_stack()))
        };
        let mut local_variables = SizedArray::<Types>::new(max_locals);
        for (index, arg) in actual_arg_types.iter().enumerate() {
            local_variables.set(index as u16, arg.clone());
        }
        let stack = Stack::<Types>::new(max_stack);
        let frame = Frame::new(
            local_variables,
            stack,
            object.class_file(),
            object_name.clone(),
            name,
            descriptor,
            return_type,
        );

        if method.is_synchronized() {
            unimplemented!("INVOKESTATIC: synchronized")
        }

        if method.is_native() {
            return InstructionResult::call(ExecutionContext::new(
                frame,
                InstructionStream::new_native(),
            ));
        }

        InstructionResult::call(ExecutionContext::new(
            frame,
            method.instruction_stream().clone(),
        ))
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        let index = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;
        let (object, method) = match runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefMethodOfClass(symref) => {
                let object = ObjectManager::get(symref.class_ref().name().as_str());
                object
                    .get_method(symref.name().as_str(), symref.descriptor().as_str())
                    .unwrap()
            }
            RuntimeConstants::SymRefMethodOfInterface(symref) => {
                let object = ObjectManager::get(symref.class_ref().name().as_str());
                object
                    .get_method(symref.name().as_str(), symref.descriptor().as_str())
                    .unwrap()
            }
            _ => panic!("Expected SymRefMethodOfClass or SymRefMethodOfInterface"),
        };
        format!(
            "invokestatic {}.{}:{}",
            object.name(),
            method.name(),
            method.descriptor()
        )
    }
}
