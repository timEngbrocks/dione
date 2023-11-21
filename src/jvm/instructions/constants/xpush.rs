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
pub struct BIPUSH {
    byte: U1,
}
impl Instruction for BIPUSH {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::BIPUSH));
        let byte = parser.consume_u1();
        BIPUSH { byte }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(bitutils::sign_extend32(self.byte as u32, 8));
        execution_context.stack().push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("bipush {}", self.byte)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SIPUSH {
    byte1: U1,
    byte2: U1,
}
impl Instruction for SIPUSH {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::SIPUSH));
        let byte1 = parser.consume_u1();
        let byte2 = parser.consume_u1();
        SIPUSH { byte1, byte2 }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let intermediate_short = ((self.byte1 as u16) << 8) | self.byte2 as u16;
        let value = Int::from_value(bitutils::sign_extend32(intermediate_short as u32, 16));
        execution_context.stack().push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("sipush {}, {}", self.byte1, self.byte2)
    }
}
