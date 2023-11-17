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
pub struct TABLESWITCH {
    padding: usize,
    default: i32,
    low: i32,
    high: i32,
    jump_offsets: Vec<i32>,
}
impl Instruction for TABLESWITCH {
    fn new(parser: &mut Parser) -> Self
    where
        Self: Sized,
    {
        let opcode = parser.consume_u1();
        assert_eq!(opcode, opcodes!(Instructions::TABLESWITCH));
        let padding = parser.align(4);
        let defaultbyte1 = parser.consume_u1();
        let defaultbyte2 = parser.consume_u1();
        let defaultbyte3 = parser.consume_u1();
        let defaultbyte4 = parser.consume_u1();
        let default = (defaultbyte1 as i32) << 24
            | (defaultbyte2 as i32) << 16
            | (defaultbyte3 as i32) << 8
            | defaultbyte4 as i32;
        let lowbyte1 = parser.consume_u1();
        let lowbyte2 = parser.consume_u1();
        let lowbyte3 = parser.consume_u1();
        let lowbyte4 = parser.consume_u1();
        let low = (lowbyte1 as i32) << 24
            | (lowbyte2 as i32) << 16
            | (lowbyte3 as i32) << 8
            | lowbyte4 as i32;
        let highbyte1 = parser.consume_u1();
        let highbyte2 = parser.consume_u1();
        let highbyte3 = parser.consume_u1();
        let highbyte4 = parser.consume_u1();
        let high = (highbyte1 as i32) << 24
            | (highbyte2 as i32) << 16
            | (highbyte3 as i32) << 8
            | highbyte4 as i32;
        assert!(high >= low);
        let num_jump_offsets = high - low + 1;
        let mut jump_offsets = Vec::with_capacity(num_jump_offsets as usize);
        for _ in 0..num_jump_offsets {
            let offsetbyte1 = parser.consume_u1();
            let offsetbyte2 = parser.consume_u1();
            let offsetbyte3 = parser.consume_u1();
            let offsetbyte4 = parser.consume_u1();
            let offset = (offsetbyte1 as i32) << 24
                | (offsetbyte2 as i32) << 16
                | (offsetbyte3 as i32) << 8
                | offsetbyte4 as i32;
            jump_offsets.push(offset);
        }
        TABLESWITCH {
            padding,
            default,
            low,
            high,
            jump_offsets,
        }
    }

    fn execute(&self, _: &mut Frame) -> InstructionResult {
        unimplemented!()
    }

    fn length(&self) -> U2 {
        (1 + self.padding as u32 + 12 + 4 * (self.high - self.low + 1) as u32) as u16
    }

    fn to_string(&self, _runtime_constant_pool: &RuntimeConstantPool) -> String {
        format!(
            "tableswitch {}, {}, {}, {}, {:?}",
            self.padding, self.default, self.high, self.low, self.jump_offsets
        )
    }
}
