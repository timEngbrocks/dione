use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{BranchKind, Instruction, InstructionResult}, runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GOTO_W {
    branchbyte1: U1,
    branchbyte2: U1,
    branchbyte3: U1,
    branchbyte4: U1,
}
impl Instruction for GOTO_W {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::GOTO_W));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        let branchbyte3 = parser.consume_u1();
        let branchbyte4 = parser.consume_u1();
        GOTO_W {
            branchbyte1,
            branchbyte2,
            branchbyte3,
            branchbyte4,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i32) << 24)
            | ((self.branchbyte2 as i32) << 16)
            | ((self.branchbyte3 as i32) << 8)
            | self.branchbyte4 as i32;
        InstructionResult::branch(BranchKind::Relative(offset))
    }

    fn length(&self) -> U2 {
        5
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!(
            "goto_w: {}, {}, {}, {}",
            self.branchbyte1, self.branchbyte2, self.branchbyte3, self.branchbyte4
        )
    }
}
