use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{long::Long, Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LCONST_0 {}
impl Instruction for LCONST_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LCONST_0));
        LCONST_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Long::from_value(0);
        execution_context.stack.push(Types::Long(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lconst_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LCONST_1 {}
impl Instruction for LCONST_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LCONST_1));
        LCONST_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Long::from_value(1);
        execution_context.stack.push(Types::Long(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lconst_1")
    }
}
