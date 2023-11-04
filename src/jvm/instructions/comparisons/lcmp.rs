use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        types::{int::Int, Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LCMP {}
impl Instruction for LCMP {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LCMP));
        LCMP {}
    }

    #[allow(clippy::if_same_then_else)]
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Long(value2) => match execution_context.stack.pop() {
                Types::Long(value1) => {
                    let value1 = value1.get();
                    let value2 = value2.get();
                    let result = match value1.cmp(&value2) {
                        std::cmp::Ordering::Greater => 1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Less => -1,
                    };
                    execution_context
                        .stack
                        .push(Types::Int(Int::from_value(result)));
                    InstructionResult::empty()
                }
                _ => panic!("Expected Long"),
            },
            _ => panic!("Expected Long"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("lcmp")
    }
}
