use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ACMPEQ {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IF_ACMPEQ {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IF_ACMPEQ));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IF_ACMPEQ {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("IF_ACMPEQ::execute")
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("if_acmpeq: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IF_ACMPNE {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IF_ACMPNE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IF_ACMPNE));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IF_ACMPNE {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("IF_ACMPNE::execute")
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("if_acmpne: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
