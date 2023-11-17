use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{int::Int, Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IINC {
    index: U1,
    constant: U1,
}
impl Instruction for IINC {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IINC));
        let index = parser.consume_u1();
        let constant = parser.consume_u1();
        IINC { index, constant }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let constant = self.constant as i32;
        match execution_context
            .local_variables
            .get(self.index as u16)
            .clone()
        {
            Types::Int(value) => {
                execution_context.local_variables.set(
                    self.index as u16,
                    Types::Int(Int::from_value(value.get() + constant)),
                );
                InstructionResult::empty()
            }
            _ => panic!("Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("iinc {} {}", self.index, self.constant)
    }
}
