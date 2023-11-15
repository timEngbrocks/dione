use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{frame::Frame, runtime_constant_pool::RuntimeConstantPool, instructions::{Instruction, InstructionResult}, native::native_call},
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IMPDEP1 {}
impl IMPDEP1 {
	pub fn new_native_call() -> IMPDEP1 {
		IMPDEP1 {}
	}
}
impl Instruction for IMPDEP1 {
    fn new(_: &mut Parser) -> Self
    where
        Self: Sized,
    {
        panic!("IMPDEP1 found in InstructionStream but can only be created by dione")
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        native_call(execution_context)
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("impdep1 -> native call")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IMPDEP2 {}
impl Instruction for IMPDEP2 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IMPDEP2));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        unimplemented!("IMPDEP2")
    }
}