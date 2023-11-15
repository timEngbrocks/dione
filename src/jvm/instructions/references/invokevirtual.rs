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
    util::{sized_array::SizedArray, stack::Stack},
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKEVIRTUAL {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for INVOKEVIRTUAL {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::INVOKEVIRTUAL));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        INVOKEVIRTUAL {
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
        let object_ref = match execution_context.stack.pop() {
            Types::Reference(object_ref) => object_ref,
            _ => panic!("INVOKEVIRTUAL: Expected Reference for object_ref"),
        };

        if method.is_synchronized() {
            unimplemented!("INVOKEVIRTUAL: synchronized")
        }

        if method.is_varargs() && method.is_native() {
            unimplemented!(
                "INVOKEVIRTUAL: varargs and native -> potentially signature polymorphic method"
            )
        }

        if method.is_native() {
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
        local_variables.set(0, Types::Reference(object_ref.clone()));
        for (index, arg) in arg_types.iter().enumerate() {
            local_variables.set((index + 1) as u16, arg.clone());
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
            "invokevirtual {}.{}:{}",
            object.name, method.name, method.descriptor
        )
    }
}
