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
pub struct ALOAD_0 {}
impl Instruction for ALOAD_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ALOAD_0));
        ALOAD_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(0).clone();
        assert!(value.is_referenceable());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("aload_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ALOAD_1 {}
impl Instruction for ALOAD_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ALOAD_1));
        ALOAD_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(1).clone();
        assert!(value.is_referenceable());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("aload_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ALOAD_2 {}
impl Instruction for ALOAD_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ALOAD_2));
        ALOAD_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(2).clone();
        assert!(value.is_referenceable());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("aload_2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ALOAD_3 {}
impl Instruction for ALOAD_3 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ALOAD_3));
        ALOAD_3 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context.local_variables().get(3).clone();
        assert!(value.is_referenceable());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("aload_3")
    }
}
