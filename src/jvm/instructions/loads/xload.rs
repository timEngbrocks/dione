use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ILOAD {
    index: U1,
}
impl Instruction for ILOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ILOAD));
        let index = parser.consume_u1();
        ILOAD { index }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context
            .local_variables()
            .get(self.index as u16)
            .clone();
        assert!(value.is_int());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("iload {}", self.index)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LLOAD {
    index: U1,
}
impl Instruction for LLOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LLOAD));
        let index = parser.consume_u1();
        LLOAD { index }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context
            .local_variables()
            .get(self.index as u16)
            .clone();
        assert!(value.is_long());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("lload {}", self.index)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FLOAD {
    index: U1,
}
impl Instruction for FLOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FLOAD));
        let index = parser.consume_u1();
        FLOAD { index }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context
            .local_variables()
            .get(self.index as u16)
            .clone();
        assert!(value.is_float());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("fload {}", self.index)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DLOAD {
    index: U1,
}
impl Instruction for DLOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DLOAD));
        let index = parser.consume_u1();
        DLOAD { index }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context
            .local_variables()
            .get(self.index as u16)
            .clone();
        assert!(value.is_double());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("dload {}", self.index)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ALOAD {
    index: U1,
}
impl Instruction for ALOAD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ALOAD));
        let index = parser.consume_u1();
        ALOAD { index }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = execution_context
            .local_variables()
            .get(self.index as u16)
            .clone();
        assert!(value.is_referenceable());
        execution_context.stack().push(value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        2
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("aload {}", self.index)
    }
}
