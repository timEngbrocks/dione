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
pub struct LSTORE_0 {}
impl Instruction for LSTORE_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LSTORE_0));
        LSTORE_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Long(value) => {
                execution_context.local_variables.set(0, Types::Long(value));
                InstructionResult::empty()
            }
            _ => panic!("LSTORE_0: Expected Long"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("lstore_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE_1 {}
impl Instruction for LSTORE_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LSTORE_1));
        LSTORE_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Long(value) => {
                execution_context.local_variables.set(1, Types::Long(value));
                InstructionResult::empty()
            }
            _ => panic!("LSTORE_1: Expected Long"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("lstore_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE_2 {}
impl Instruction for LSTORE_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LSTORE_2));
        LSTORE_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Long(value) => {
                execution_context.local_variables.set(2, Types::Long(value));
                InstructionResult::empty()
            }
            _ => panic!("LSTORE_2: Expected Long"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("lstore_2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSTORE_3 {}
impl Instruction for LSTORE_3 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LSTORE_3));
        LSTORE_3 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.stack.pop();
        match value {
            Types::Long(value) => {
                execution_context.local_variables.set(3, Types::Long(value));
                InstructionResult::empty()
            }
            _ => panic!("LSTORE_3: Expected Long"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("lstore_3")
    }
}
