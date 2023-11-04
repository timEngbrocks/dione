use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        types::Types,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISTORE_0 {}
impl Instruction for ISTORE_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ISTORE_0));
        ISTORE_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Int(value) => {
                execution_context.local_variables.set(0, Types::Int(value));
                InstructionResult::empty()
            }
            _ => panic!("ISTORE_0: Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("istore_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISTORE_1 {}
impl Instruction for ISTORE_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ISTORE_1));
        ISTORE_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Int(value) => {
                execution_context.local_variables.set(1, Types::Int(value));
                InstructionResult::empty()
            }
            _ => panic!("ISTORE_1: Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("istore_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISTORE_2 {}
impl Instruction for ISTORE_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ISTORE_2));
        ISTORE_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Int(value) => {
                execution_context.local_variables.set(2, Types::Int(value));
                InstructionResult::empty()
            }
            _ => panic!("ISTORE_2: Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("istore_2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISTORE_3 {}
impl Instruction for ISTORE_3 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ISTORE_3));
        ISTORE_3 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Int(value) => {
                execution_context.local_variables.set(3, Types::Int(value));
                InstructionResult::empty()
            }
            _ => panic!("ISTORE_3: Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("istore_3")
    }
}
