use core::panic;

use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        descriptor::parse_method_descriptor,
        execution_context::ExecutionContext,
        frame::Frame,
        instructions::{Instruction, InstructionResult, InstructionStream},
        object_manager::ObjectManager,
        runtime_constant_pool::{
            sym_ref_method_of_class::SymRefMethodOfClass,
            sym_ref_method_of_interface::SymRefMethodOfInterface, RuntimeConstants, RuntimeConstantPool,
        },
        types::Types,
    },
    opcodes,
    util::{sized_array::SizedArray, stack::Stack, heap::Heap},
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
        let (object, method) = match execution_context.runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefMethodOfClass(SymRefMethodOfClass {
                name,
                descriptor,
                class_ref,
            }) => {
                let object = ObjectManager::get(class_ref.name.as_str());
                object
                    .get_method(name.as_str(), descriptor.as_str())
                    .unwrap()
            }
            RuntimeConstants::SymRefMethodOfInterface(SymRefMethodOfInterface {
                name,
                descriptor,
                class_ref,
            }) => {
                let object = ObjectManager::get(class_ref.name.as_str());
                object
                    .get_method(name.as_str(), descriptor.as_str())
                    .unwrap()
            }
            _ => panic!("Expected SymRefMethodOfClass or SymRefMethodOfInterface"),
        };
        let parse_result = parse_method_descriptor(&method.descriptor);
        let (mut arg_types, return_type) = match parse_result {
            Some((arg_types, return_type)) => (arg_types, return_type),
            None => panic!("Invalid method descriptor: {}", method.descriptor),
        };
        arg_types = arg_types
            .iter()
            .rev()
            .map(|arg| {
                let actual_arg = execution_context.stack.pop();
                actual_arg.assert_matches_type(arg);
                actual_arg
            })
            .collect::<Vec<Types>>();

        if !method.is_static() || method.is_abstract() {
            panic!("Expected static method")
        }

        ObjectManager::initialize_object(&object.name);

        if method.is_synchronized() {
            unimplemented!("INVOKESTATIC: synchronized")
        }

        if method.is_native() {
            /*
            NOTE: native_call needs access to &Object on which the native method is being called.
            There are different scenarios for &Object's origin depending on the invocation method:
            INVOKESTATIC: &Object needs to come directly from ObjectManager
            INVOKEVIRTUAL: &Object needs to come from the stack
            INVOKEINTERFACE: &Object needs to come from the stack
            INVOKESPECIAL: &Object needs to come from the stack

            FIXME: Find a way to do this

            I really like the idea of using impdep1 as a trap door for native calls.
            Maybe we can just put all of the information on the stack and make native variants of the different invocation methods.
            For this we would need a global getter for getting a native method implementation that returns a functor which can
            be called by the invocation method's native implementation.
            */
            return InstructionResult::call(ExecutionContext::new(
                Frame::new_native(&object.class_file, index.clone(), object.name.clone(), method.name.clone(), method.descriptor.clone(), return_type),
                InstructionStream::new_native(),
            ));
        }

        let (max_locals, max_stack) = if method.is_native() {
            (None, None)
        } else {
            (Some(method.max_locals), Some(method.max_stack))
        };
        let mut local_variables = SizedArray::<Types>::new(max_locals);
        for (index, arg) in arg_types.iter().enumerate() {
            local_variables.set(index as u16, arg.clone());
        }
        let stack = Stack::<Types>::new(max_stack);
        let frame = Frame::new(
            local_variables,
            stack,
            &object.class_file,
            index.clone(),
            object.name.clone(),
            method.name.clone(),
            method.descriptor.clone(),
            return_type,
        );

        InstructionResult::call(ExecutionContext::new(
            frame,
            method.instruction_stream.clone(),
        ))
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, runtime_constant_pool: &RuntimeConstantPool) -> String {
        let index = ((self.indexbyte1 as U2) << 8) | self.indexbyte2 as U2;
        let (object, method) = match runtime_constant_pool.get(index) {
            RuntimeConstants::SymRefMethodOfClass(SymRefMethodOfClass {
                name,
                descriptor,
                class_ref,
            }) => {
                let object = ObjectManager::get(class_ref.name.as_str());
                object
                    .get_method(name.as_str(), descriptor.as_str())
                    .unwrap()
            }
            RuntimeConstants::SymRefMethodOfInterface(SymRefMethodOfInterface {
                name,
                descriptor,
                class_ref,
            }) => {
                let object = ObjectManager::get(class_ref.name.as_str());
                object
                    .get_method(name.as_str(), descriptor.as_str())
                    .unwrap()
            }
            _ => panic!("Expected SymRefMethodOfClass or SymRefMethodOfInterface"),
        };
        format!(
            "invokestatic {}.{}:{}",
            object.name, method.name, method.descriptor
        )
    }
}
