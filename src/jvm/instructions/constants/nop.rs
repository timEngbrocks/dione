use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult, Instructions},
        runtime_constant_pool::RuntimeConstantPool,
    },
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct NOP {}

impl Instruction for NOP {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert!(opcode == (Instructions::NOP as u8));
        NOP {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("nop")
    }
}
