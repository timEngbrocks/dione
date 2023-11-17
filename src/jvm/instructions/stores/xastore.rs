use crate::{
    class_loader::parser::{Parser, U2},
    jvm::{
        frame::Frame,
        instructions::{Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{array::Array, Types, Value},
    },
    opcodes,
    util::heap::{ArrayPtr, ReferencePtr},
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IASTORE {}
impl Instruction for IASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IASTORE));
        IASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Int(value) => value,
            _ => panic!("IASTORE: value must be an int"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("IASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("IASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Primitive(array) => array.borrow_mut(),
                _ => panic!("IASTORE: array must be an array of ints"),
            },
            _ => panic!("IASTORE: array must be an array"),
        };
        array.set(index, Types::Int(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("iastore")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct LASTORE {}
impl Instruction for LASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::LASTORE));
        LASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Long(value) => value,
            _ => panic!("LASTORE: value must be a long"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("LASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("LASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Primitive(array) => array.borrow_mut(),
                _ => panic!("LASTORE: array must be an array of longs"),
            },
            _ => panic!("LASTORE: array must be an array"),
        };
        array.set(index, Types::Long(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("lastore")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct FASTORE {}
impl Instruction for FASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::FASTORE));
        FASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Float(value) => value,
            _ => panic!("FASTORE: value must be a float"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("FASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("FASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Primitive(array) => array.borrow_mut(),
                _ => panic!("FASTORE: array must be an array of floats"),
            },
            _ => panic!("FASTORE: array must be an array"),
        };
        array.set(index, Types::Float(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("fastore")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct DASTORE {}
impl Instruction for DASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::DASTORE));
        DASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Double(value) => value,
            _ => panic!("DASTORE: value must be a double"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("DASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("DASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Primitive(array) => array.borrow_mut(),
                _ => panic!("DASTORE: array must be an array of doubles"),
            },
            _ => panic!("DASTORE: array must be an array"),
        };
        array.set(index, Types::Double(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("dastore")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct AASTORE {}
impl Instruction for AASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::AASTORE));
        AASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("AASTORE: value must be a reference"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("AASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("AASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Reference(array) => array.borrow_mut(),
                _ => panic!("AASTORE: array must be an array of references"),
            },
            _ => panic!("AASTORE: array must be an array"),
        };
        array.set_helper(index, value);
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("aastore")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BASTORE {}
impl Instruction for BASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::BASTORE));
        BASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Byte(value) => value,
            _ => panic!("BASTORE: value must be a byte"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("BASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("BASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Primitive(array) => array.borrow_mut(),
                _ => panic!("BASTORE: array must be an array of bytes"),
            },
            _ => panic!("BASTORE: array must be an array"),
        };
        array.set(index, Types::Byte(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("bastore")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CASTORE {}
impl Instruction for CASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::CASTORE));
        CASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Char(value) => value,
            _ => panic!("CASTORE: value must be a char"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("CASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("CASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Primitive(array) => array.borrow_mut(),
                _ => panic!("CASTORE: array must be an array of chars"),
            },
            _ => panic!("CASTORE: array must be an array"),
        };
        array.set(index, Types::Char(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("castore")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct SASTORE {}
impl Instruction for SASTORE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::SASTORE));
        SASTORE {}
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let value = match execution_context.stack.pop() {
            Types::Short(value) => value,
            _ => panic!("SASTORE: value must be a short"),
        };
        let index = match execution_context.stack.pop() {
            Types::Int(index) => index.get() as usize,
            _ => panic!("SASTORE: index must be an int"),
        };
        let arrayref = match execution_context.stack.pop() {
            Types::Reference(reference) => reference,
            _ => panic!("SASTORE: arrayref must be a reference"),
        };
        let mut array = arrayref.get();
        let mut array = match array {
            ReferencePtr::Array(ref mut array) => match array {
                ArrayPtr::Primitive(array) => array.borrow_mut(),
                _ => panic!("SASTORE: array must be an array of shorts"),
            },
            _ => panic!("SASTORE: array must be an array"),
        };
        array.set(index, Types::Short(value));
        InstructionResult::empty()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        String::from("sastore")
    }
}
