use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{double::Double, float::Float, int::Int, long::Long, Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ISUB {}
impl Instruction for ISUB {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ISUB));
        ISUB {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Int(value2) => match execution_context.stack.pop() {
                Types::Int(value1) => {
                    let value1 = value1.get();
                    let value2 = value2.get();
                    let result = value1 - value2;
                    execution_context
                        .stack
                        .push(Types::Int(Int::from_value(result)));
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

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("isub")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LSUB {}
impl Instruction for LSUB {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LSUB));
        LSUB {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Long(value2) => match execution_context.stack.pop() {
                Types::Long(value1) => {
                    let value1 = value1.get();
                    let value2 = value2.get();
                    let result = value1 - value2;
                    execution_context
                        .stack
                        .push(Types::Long(Long::from_value(result)));
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

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lsub")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FSUB {}
impl Instruction for FSUB {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FSUB));
        FSUB {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Float(value2) => match execution_context.stack.pop() {
                Types::Float(value1) => {
                    let value1 = value1.get();
                    let value2 = value2.get();
                    let result = value1 - value2;
                    execution_context
                        .stack
                        .push(Types::Float(Float::from_value(result)));
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
        String::from("fsub")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DSUB {}
impl Instruction for DSUB {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DSUB));
        DSUB {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Double(value2) => match execution_context.stack.pop() {
                Types::Double(value1) => {
                    let value1 = value1.get();
                    let value2 = value2.get();
                    let result = value1 - value2;
                    execution_context
                        .stack
                        .push(Types::Double(Double::from_value(result)));
                    InstructionResult::empty()
                }
                _ => panic!("Expected Double"),
            },
            _ => panic!("Expected Double"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dsub")
    }
}
