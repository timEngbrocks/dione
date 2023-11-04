use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{BranchKind, Instruction, InstructionResult},
        types::Types,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFNULL {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IFNULL {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IFNULL));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IFNULL {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        if let Types::Reference(reference) = execution_context.stack.pop() {
            if reference.is_null() {
                let offset = (self.branchbyte1 as i16) << 8 | self.branchbyte2 as i16;
                InstructionResult::branch(BranchKind::Relative(offset as i32))
            } else {
                InstructionResult::empty()
            }
        } else {
            panic!("IFNULL: Expected a Reference")
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self) -> String {
        format!(
            "ifnull {}",
            (self.branchbyte1 as i16) << 8 | self.branchbyte2 as i16
        )
    }
}
