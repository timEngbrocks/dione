use crate::{
    class_loader::parser::{Parser, U2},
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
pub struct FCMPG {}
impl Instruction for FCMPG {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FCMPG));
        FCMPG {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack().pop() {
            Types::Float(value2) => match execution_context.stack().pop() {
                Types::Float(value1) => {
                    let value1 = value1.get();
                    let value2 = value2.get();
                    let result = if value1 > value2 {
                        1
                    } else if value1 == value2 {
                        0
                    } else if value1 < value2 {
                        -1
                    } else {
                        1
                    };
                    execution_context
                        .stack()
                        .push(Types::Int(Int::from_value(result)));
                    InstructionResult::empty()
                }
                _ => panic!("Expected Float"),
            },
            _ => panic!("Expected Float"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("fcmpg")
    }
}
