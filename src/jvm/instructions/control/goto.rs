use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{BranchKind, Instruction, InstructionResult},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GOTO {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for GOTO {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::GOTO));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        GOTO {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
        InstructionResult::branch(BranchKind::Relative(offset as i32))
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self) -> String {
        format!("goto: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
