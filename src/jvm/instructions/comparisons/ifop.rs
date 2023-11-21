use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::{
        frame::Frame,
        instructions::{BranchKind, Instruction, InstructionResult},
        runtime_constant_pool::RuntimeConstantPool,
        types::{Types, Value},
    },
    opcodes,
};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFEQ {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IFEQ {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IFEQ));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IFEQ {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
        match execution_context.stack().pop() {
            Types::Int(value) => {
                if value.get() == 0 {
                    InstructionResult::branch(BranchKind::Relative(offset as i32))
                } else {
                    InstructionResult::empty()
                }
            }
            _ => panic!("IFEQ: Expected a Int"),
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("ifeq: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFNE {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IFNE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IFNE));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IFNE {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
        match execution_context.stack().pop() {
            Types::Int(value) => {
                if value.get() != 0 {
                    InstructionResult::branch(BranchKind::Relative(offset as i32))
                } else {
                    InstructionResult::empty()
                }
            }
            _ => panic!("IFNE: Expected an Int"),
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("ifne: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFLT {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IFLT {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IFLT));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IFLT {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
        match execution_context.stack().pop() {
            Types::Int(value) => {
                if value.get() < 0 {
                    InstructionResult::branch(BranchKind::Relative(offset as i32))
                } else {
                    InstructionResult::empty()
                }
            }
            _ => panic!("IFLT: Expected a Int"),
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("iflt: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFGE {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IFGE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IFGE));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IFGE {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
        match execution_context.stack().pop() {
            Types::Int(value) => {
                if value.get() >= 0 {
                    InstructionResult::branch(BranchKind::Relative(offset as i32))
                } else {
                    InstructionResult::empty()
                }
            }
            _ => panic!("IFGE: Expected a Int"),
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("ifge: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFGT {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IFGT {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IFGT));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IFGT {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
        match execution_context.stack().pop() {
            Types::Int(value) => {
                if value.get() > 0 {
                    InstructionResult::branch(BranchKind::Relative(offset as i32))
                } else {
                    InstructionResult::empty()
                }
            }
            _ => panic!("IFGT: Expected a Int"),
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("ifgt: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IFLE {
    branchbyte1: U1,
    branchbyte2: U1,
}
impl Instruction for IFLE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IFLE));
        let branchbyte1 = parser.consume_u1();
        let branchbyte2 = parser.consume_u1();
        IFLE {
            branchbyte1,
            branchbyte2,
        }
    }

    fn execute(&self, execution_context: &mut Frame) -> InstructionResult {
        let offset = ((self.branchbyte1 as i16) << 8) | self.branchbyte2 as i16;
        match execution_context.stack().pop() {
            Types::Int(value) => {
                if value.get() <= 0 {
                    InstructionResult::branch(BranchKind::Relative(offset as i32))
                } else {
                    InstructionResult::empty()
                }
            }
            _ => panic!("IFLE: Expected a Int"),
        }
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!("ifle: {}, {}", self.branchbyte1, self.branchbyte2)
    }
}
