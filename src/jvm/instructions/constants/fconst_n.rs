use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{float::Float, Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FCONST_0 {}
impl Instruction for FCONST_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FCONST_0));
        FCONST_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Float::from_value(0.0);
        execution_context.stack.push(Types::Float(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("fconst_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FCONST_1 {}
impl Instruction for FCONST_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FCONST_1));
        FCONST_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Float::from_value(1.0);
        execution_context.stack.push(Types::Float(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("fconst_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FCONST_2 {}
impl Instruction for FCONST_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FCONST_2));
        FCONST_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Float::from_value(2.0);
        execution_context.stack.push(Types::Float(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("fconst_2")
    }
}
