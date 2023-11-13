use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        types::{int::Int, Types, Value}, runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ICONST_M1 {}
impl Instruction for ICONST_M1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ICONST_M1));
        ICONST_M1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(-1);
        execution_context.stack.push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iconst_m1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ICONST_0 {}
impl Instruction for ICONST_0 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ICONST_0));
        ICONST_0 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(0);
        execution_context.stack.push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iconst_0")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ICONST_1 {}
impl Instruction for ICONST_1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ICONST_1));
        ICONST_1 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(1);
        execution_context.stack.push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iconst_1")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ICONST_2 {}
impl Instruction for ICONST_2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ICONST_2));
        ICONST_2 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(2);
        execution_context.stack.push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iconst_2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ICONST_3 {}
impl Instruction for ICONST_3 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ICONST_3));
        ICONST_3 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(3);
        execution_context.stack.push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iconst_3")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ICONST_4 {}
impl Instruction for ICONST_4 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ICONST_4));
        ICONST_4 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(4);
        execution_context.stack.push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iconst_4")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ICONST_5 {}
impl Instruction for ICONST_5 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ICONST_5));
        ICONST_5 {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = Int::from_value(5);
        execution_context.stack.push(Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iconst_5")
    }
}
