use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{int::Int, long::Long, Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IXOR {}
impl Instruction for IXOR {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IXOR));
        IXOR {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Int(value2) => match execution_context.stack.pop() {
                Types::Int(value1) => {
                    execution_context
                        .stack
                        .push(Types::Int(Int::from_value(value1.get() ^ value2.get())));
                    InstructionResult::empty()
                }
                _ => panic!("Expected Int"),
            },
            _ => panic!("Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("ixor")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LXOR {}
impl Instruction for LXOR {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LXOR));
        LXOR {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Long(value2) => match execution_context.stack.pop() {
                Types::Long(value1) => {
                    execution_context
                        .stack
                        .push(Types::Long(Long::from_value(value1.get() ^ value2.get())));
                    InstructionResult::empty()
                }
                _ => panic!("Expected Long"),
            },
            _ => panic!("Expected Long"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lxor")
    }
}
