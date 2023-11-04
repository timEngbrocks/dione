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
pub struct FLOAD_0 {}
impl Instruction for FLOAD_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FLOAD_0));
        FLOAD_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.local_variables.get(0) {
            Types::Float(value) => {
                execution_context.stack.push(Types::Float(value.clone()));
            }
            _ => panic!("FLOAD_0: Expected a Float"),
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("fload_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD_1 {}
impl Instruction for FLOAD_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FLOAD_1));
        FLOAD_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.local_variables.get(1) {
            Types::Float(value) => {
                execution_context.stack.push(Types::Float(value.clone()));
            }
            _ => panic!("FLOAD_1: Expected a Float"),
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("fload_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD_2 {}
impl Instruction for FLOAD_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FLOAD_2));
        FLOAD_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.local_variables.get(2) {
            Types::Float(value) => {
                execution_context.stack.push(Types::Float(value.clone()));
            }
            _ => panic!("FLOAD_2: Expected a Float"),
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("fload_2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD_3 {}
impl Instruction for FLOAD_3 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FLOAD_3));
        FLOAD_3 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.local_variables.get(3) {
            Types::Float(value) => {
                execution_context.stack.push(Types::Float(value.clone()));
            }
            _ => panic!("FLOAD_3: Expected a Float"),
        }
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("fload_3")
    }
}
