use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IALOAD {}
impl Instruction for IALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IALOAD));
        IALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("IALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iaload")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LALOAD {}
impl Instruction for LALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LALOAD));
        LALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("LALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("laload")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FALOAD {}
impl Instruction for FALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FALOAD));
        FALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("FALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("faload")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DALOAD {}
impl Instruction for DALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DALOAD));
        DALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("DALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("daload")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AALOAD {}
impl Instruction for AALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::AALOAD));
        AALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("AALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("aaload")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BALOAD {}
impl Instruction for BALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::BALOAD));
        BALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("BALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("baload")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CALOAD {}
impl Instruction for CALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::CALOAD));
        CALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("CALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("caload")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SALOAD {}
impl Instruction for SALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::SALOAD));
        SALOAD {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!("SALOAD")
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("saload")
    }
}
