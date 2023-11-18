use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_0 {}
impl Instruction for LLOAD_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LLOAD_0));
        LLOAD_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(0).clone();
        assert!(value.is_long());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lload_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_1 {}
impl Instruction for LLOAD_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LLOAD_1));
        LLOAD_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(1).clone();
        assert!(value.is_long());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lload_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_2 {}
impl Instruction for LLOAD_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LLOAD_2));
        LLOAD_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(2).clone();
        assert!(value.is_long());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lload_2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD_3 {}
impl Instruction for LLOAD_3 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LLOAD_3));
        LLOAD_3 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(3).clone();
        assert!(value.is_long());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lload_3")
    }
}
