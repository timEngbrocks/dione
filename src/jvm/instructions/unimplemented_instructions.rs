use crate::{
    class_loader::parser::{Parser, U1, U2},
    jvm::frame::Frame,
    opcodes,
};

use super::{Instruction, InstructionResult};

#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct JSR {}
impl Instruction for JSR {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::JSR));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("JSR")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct RET {}
impl Instruction for RET {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::RET));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("RET")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct WIDE {}
impl Instruction for WIDE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::WIDE));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("WIDE")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct GOTO_W {}
impl Instruction for GOTO_W {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::GOTO_W));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("GOTO_W")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct JSR_W {}
impl Instruction for JSR_W {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::JSR_W));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("JSR_W")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct BREAKPOINT {}
impl Instruction for BREAKPOINT {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::BREAKPOINT));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("BREAKPOINT")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct IMPDEP1 {}
impl Instruction for IMPDEP1 {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::IMPDEP1));
        todo!("Implement");
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("IMPDEP1")
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

    fn to_string(&self) -> String {
        unimplemented!("IMPDEP2")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct PUTFIELD {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for PUTFIELD {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::PUTFIELD));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        PUTFIELD {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self) -> String {
        unimplemented!("PUTFIELD")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKEINTERFACE {
    indexbyte1: U1,
    indexbyte2: U1,
    count: U1,
}
impl Instruction for INVOKEINTERFACE {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::INVOKEINTERFACE));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        let count = parser.consume_u1();
        let byte4 = parser.consume_u1();
        assert_eq!(byte4, 0);
        INVOKEINTERFACE {
            indexbyte1,
            indexbyte2,
            count,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        5
    }

    fn to_string(&self) -> String {
        unimplemented!("INVOKEINTERFACE")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INVOKEDYNAMIC {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for INVOKEDYNAMIC {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::INVOKEDYNAMIC));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        let byte3 = parser.consume_u1();
        assert_eq!(byte3, 0);
        let byte4 = parser.consume_u1();
        assert_eq!(byte4, 0);
        INVOKEDYNAMIC {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        5
    }

    fn to_string(&self) -> String {
        unimplemented!("INVOKEDYNAMIC")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct ARRAYLENGTH {}
impl Instruction for ARRAYLENGTH {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::ARRAYLENGTH));
        ARRAYLENGTH {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("ARRAYLENGTH")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct CHECKCAST {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for CHECKCAST {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::CHECKCAST));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        CHECKCAST {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self) -> String {
        unimplemented!("CHECKCAST")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct INSTANCEOF {
    indexbyte1: U1,
    indexbyte2: U1,
}
impl Instruction for INSTANCEOF {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::INSTANCEOF));
        let indexbyte1 = parser.consume_u1();
        let indexbyte2 = parser.consume_u1();
        INSTANCEOF {
            indexbyte1,
            indexbyte2,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        3
    }

    fn to_string(&self) -> String {
        unimplemented!("INSTANCEOF")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MONITORENTER {}
impl Instruction for MONITORENTER {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::MONITORENTER));
        MONITORENTER {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("MONITORENTER")
    }
}
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct MONITOREXIT {}
impl Instruction for MONITOREXIT {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::MONITOREXIT));
        MONITOREXIT {}
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        1
    }

    fn to_string(&self) -> String {
        unimplemented!("MONITOREXIT")
    }
}
