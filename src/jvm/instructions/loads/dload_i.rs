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
pub struct DLOAD_0 {}
impl Instruction for DLOAD_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DLOAD_0));
        DLOAD_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(0).clone();
        assert!(value.is_double());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dload_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD_1 {}
impl Instruction for DLOAD_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DLOAD_1));
        DLOAD_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(1).clone();
        assert!(value.is_double());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dload_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD_2 {}
impl Instruction for DLOAD_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DLOAD_2));
        DLOAD_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(2).clone();
        assert!(value.is_double());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dload_2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD_3 {}
impl Instruction for DLOAD_3 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DLOAD_3));
        DLOAD_3 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(3).clone();
        assert!(value.is_double());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dload_3")
    }
}
