use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{
            conversions::x2y::{i2b, i2c, i2s},
            Instruction, InstructionResult,
        },
        types::{
            double::Double, float::Float, int::Int, long::Long, reference::Reference, ReturnTypes,
            Types, Value,
        }, runtime_constant_pool::RuntimeConstantPool,
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IRETURN {}
impl Instruction for IRETURN {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IRETURN));
        IRETURN {}
    }

    // TODO: Monitor update
    // TODO: Exception handling
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Int(value) => ireturn(value, &execution_context.return_value),
            _ => panic!("Expected Int"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("ireturn")
    }
}

pub fn ireturn(value: Int, return_type: &ReturnTypes) -> InstructionResult {
    let return_value = match return_type {
        ReturnTypes::Type(t) => match t {
            Types::Byte(_) => Types::Byte(i2b(value)),
            Types::Char(_) => Types::Char(i2c(value)),
            Types::Short(_) => Types::Short(i2s(value)),
            Types::Boolean(_) => Types::Int(Int::from_value(value.get() & 1)),
            Types::Int(_) => Types::Int(value),
            _ => panic!("Expected Int return type"),
        },
        ReturnTypes::Void => panic!("Expected Int return type"),
    };
    InstructionResult::return_value(return_value)
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LRETURN {}
impl Instruction for LRETURN {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LRETURN));
        LRETURN {}
    }

    // TODO: Monitor update
    // TODO: Exception handling
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Long(value) => lreturn(value, &execution_context.return_value),
            _ => panic!("Expected Long"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lreturn")
    }
}

pub fn lreturn(value: Long, _: &ReturnTypes) -> InstructionResult {
    InstructionResult::return_value(Types::Long(value))
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FRETURN {}
impl Instruction for FRETURN {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FRETURN));
        FRETURN {}
    }

    // TODO: Monitor update
    // TODO: Exception handling
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Float(value) => freturn(value, &execution_context.return_value),
            _ => panic!("Expected Float"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("freturn")
    }
}

pub fn freturn(value: Float, _: &ReturnTypes) -> InstructionResult {
    InstructionResult::return_value(Types::Float(value))
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DRETURN {}
impl Instruction for DRETURN {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DRETURN));
        DRETURN {}
    }

    // TODO: Monitor update
    // TODO: Exception handling
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            Types::Double(value) => dreturn(value, &execution_context.return_value),
            _ => panic!("Expected Double"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dreturn")
    }
}

pub fn dreturn(value: Double, _: &ReturnTypes) -> InstructionResult {
    InstructionResult::return_value(Types::Double(value))
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ARETURN {}
impl Instruction for ARETURN {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ARETURN));
        ARETURN {}
    }

    // TODO: Monitor update
    // TODO: Exception handling
    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        match execution_context.stack.pop() {
            // FIXME: Check that the reference on the stack and the return type are assignment compatible
            Types::Reference(value) => areturn(value, &execution_context.return_value),
            _ => panic!("Expected Reference"),
        }
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("areturn")
    }
}

pub fn areturn(value: Reference, _: &ReturnTypes) -> InstructionResult {
    InstructionResult::return_value(Types::Reference(value))
}

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RETURN {}
impl Instruction for RETURN {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::RETURN));
        RETURN {}
    }

    // TODO: Monitor update
    // TODO: Exception handling
    fn execute(&self, _: &mut Frame) -> InstructionResult {
        InstructionResult::return_void()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("return")
    }
}
