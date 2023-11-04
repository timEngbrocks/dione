use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        types::{int::Int, long::Long, Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IUSHR {}
impl Instruction for IUSHR {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IUSHR));
        IUSHR {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Int(value2) => match execution_context.stack.pop() {
                Types::Int(value1) => {
                    let s = value2.get() & 0b11111;
                    let x = value1.get();
                    if x < 0 {
                        execution_context
                            .stack
                            .push(Types::Int(Int::from_value((x >> s) + (2 << !s))));
                    } else {
                        execution_context
                            .stack
                            .push(Types::Int(Int::from_value(x >> s)));
                    }
                    InstructionResult::empty()
                }
                _ => panic!("Expected Int"),
            },
            _ => panic!("Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("iushr")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LUSHR {}
impl Instruction for LUSHR {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LUSHR));
        LUSHR {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Int(value2) => match execution_context.stack.pop() {
                Types::Int(value1) => {
                    let s = value2.get() & 0b111111;
                    let x = value1.get();
                    if x < 0 {
                        execution_context
                            .stack
                            .push(Types::Long(Long::from_value(((x as i64) >> s) + (2 << !s))));
                    } else {
                        execution_context
                            .stack
                            .push(Types::Long(Long::from_value((x as i64) >> s)));
                    }
                    InstructionResult::empty()
                }
                _ => panic!("Expected Int"),
            },
            _ => panic!("Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        String::from("lushr")
    }
}
