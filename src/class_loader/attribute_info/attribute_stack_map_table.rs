use crate::class_loader::{
    constant_pool_info::ConstantPool,
    parser::{Parser, U1, U2, U4},
};

use super::Attribute;

#[derive(Debug, Clone)]
pub struct AttributeStackMapTable {
    attribute_name_index: U2,
    attribute_length: U4,
    number_of_entries: U2,
    entries: Vec<StackMapFrameTypes>,
}

impl Attribute for AttributeStackMapTable {
    fn new(parser: &mut Parser, _: &ConstantPool) -> AttributeStackMapTable {
        let attribute_name_index = parser.consume_u2();
        let attribute_length = parser.consume_u4();
        let number_of_entries = parser.consume_u2();
        let mut entries = Vec::with_capacity(number_of_entries as usize);
        for _ in 0..number_of_entries {
            entries.push(StackMapFrameTypes::new(parser));
        }

        AttributeStackMapTable {
            attribute_name_index,
            attribute_length,
            number_of_entries,
            entries,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StackMapFrameTypes {
    Same(StackMapFrameSame),
    SameLocal1StackItem(StackMapFrameSameLocal1StackItem),
    SameLocal1StackItemFrameExtended(StackMapFrameSameLocal1StackItemFrameExtended),
    Chop(StackMapFrameChop),
    SameFrameExtended(StackMapFrameSameFrameExtended),
    Append(StackMapFrameAppend),
    FullFrame(StackMapFrameFull),
}

impl StackMapFrameTypes {
    pub fn new(parser: &mut Parser) -> StackMapFrameTypes {
        let tag = parser.peek_u1();
        match tag {
            0..64 => StackMapFrameTypes::Same(StackMapFrameSame::new(parser)),
            64..128 => StackMapFrameTypes::SameLocal1StackItem(
                StackMapFrameSameLocal1StackItem::new(parser),
            ),
            247 => StackMapFrameTypes::SameLocal1StackItemFrameExtended(
                StackMapFrameSameLocal1StackItemFrameExtended::new(parser),
            ),
            248..251 => StackMapFrameTypes::Chop(StackMapFrameChop::new(parser)),
            251 => {
                StackMapFrameTypes::SameFrameExtended(StackMapFrameSameFrameExtended::new(parser))
            }
            252..255 => StackMapFrameTypes::Append(StackMapFrameAppend::new(parser)),
            255 => StackMapFrameTypes::FullFrame(StackMapFrameFull::new(parser)),
            _ => panic!("{tag}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameSame {
    frame_type: U1,
}

impl StackMapFrameSame {
    pub fn new(parser: &mut Parser) -> StackMapFrameSame {
        let frame_type = parser.consume_u1();

        StackMapFrameSame { frame_type }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameSameLocal1StackItem {
    frame_type: U1,
    stack: Vec<VerificationTypes>,
}

impl StackMapFrameSameLocal1StackItem {
    pub fn new(parser: &mut Parser) -> StackMapFrameSameLocal1StackItem {
        let frame_type = parser.consume_u1();
        let stack = vec![VerificationTypes::new(parser)];

        StackMapFrameSameLocal1StackItem { frame_type, stack }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameSameLocal1StackItemFrameExtended {
    frame_type: U1,
    offset_delta: U2,
    stack: Vec<VerificationTypes>,
}

impl StackMapFrameSameLocal1StackItemFrameExtended {
    pub fn new(parser: &mut Parser) -> StackMapFrameSameLocal1StackItemFrameExtended {
        let frame_type = parser.consume_u1();
        let offset_delta = parser.consume_u2();
        let stack = vec![VerificationTypes::new(parser)];

        StackMapFrameSameLocal1StackItemFrameExtended {
            frame_type,
            offset_delta,
            stack,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameChop {
    frame_type: U1,
    offset_delta: U2,
}

impl StackMapFrameChop {
    pub fn new(parser: &mut Parser) -> StackMapFrameChop {
        let frame_type = parser.consume_u1();
        let offset_delta = parser.consume_u2();

        StackMapFrameChop {
            frame_type,
            offset_delta,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameSameFrameExtended {
    frame_type: U1,
    offset_delta: U2,
}

impl StackMapFrameSameFrameExtended {
    pub fn new(parser: &mut Parser) -> StackMapFrameSameFrameExtended {
        let frame_type = parser.consume_u1();
        let offset_delta = parser.consume_u2();

        StackMapFrameSameFrameExtended {
            frame_type,
            offset_delta,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameAppend {
    frame_type: U1,
    offset_delta: U2,
    locals: Vec<VerificationTypes>,
}

impl StackMapFrameAppend {
    pub fn new(parser: &mut Parser) -> StackMapFrameAppend {
        let frame_type = parser.consume_u1();
        let offset_delta = parser.consume_u2();
        let mut locals = Vec::with_capacity((frame_type - 251) as usize);
        for _ in 0..(frame_type - 251) {
            locals.push(VerificationTypes::new(parser));
        }

        StackMapFrameAppend {
            frame_type,
            offset_delta,
            locals,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameFull {
    frame_type: U1,
    offset_delta: U2,
    number_of_locals: U2,
    locals: Vec<VerificationTypes>,
    number_of_stack_items: U2,
    stack: Vec<VerificationTypes>,
}

impl StackMapFrameFull {
    pub fn new(parser: &mut Parser) -> StackMapFrameFull {
        let frame_type = parser.consume_u1();
        let offset_delta = parser.consume_u2();
        let number_of_locals = parser.consume_u2();
        let mut locals = Vec::with_capacity(number_of_locals as usize);
        for _ in 0..number_of_locals {
            locals.push(VerificationTypes::new(parser));
        }
        let number_of_stack_items = parser.consume_u2();
        let mut stack = Vec::with_capacity(number_of_stack_items as usize);
        for _ in 0..number_of_stack_items {
            stack.push(VerificationTypes::new(parser));
        }

        StackMapFrameFull {
            frame_type,
            offset_delta,
            number_of_locals,
            locals,
            number_of_stack_items,
            stack,
        }
    }
}

#[derive(Debug, Clone)]
pub enum VerificationTypes {
    ItemTop(VerificationTypeTopVariable),
    ItemInteger(VerificationTypeIntegerVariable),
    ItemFloat(VerificationTypeFloatVariable),
    ItemDouble(VerificationTypeDoubleVariable),
    ItemLong(VerificationTypeLongVariable),
    ItemNull(VerificationTypeNullVariable),
    ItemUninitializedThis(VerificationTypeUninitializedThisVariable),
    ItemObject(VerificationTypeObjectVariable),
    ItemUninitialized(VerificationTypeUninitializedVariable),
}

impl VerificationTypes {
    pub fn new(parser: &mut Parser) -> VerificationTypes {
        let tag = parser.peek_u1();
        match tag {
            0 => VerificationTypes::ItemTop(VerificationTypeTopVariable::new(parser)),
            1 => VerificationTypes::ItemInteger(VerificationTypeIntegerVariable::new(parser)),
            2 => VerificationTypes::ItemFloat(VerificationTypeFloatVariable::new(parser)),
            3 => VerificationTypes::ItemDouble(VerificationTypeDoubleVariable::new(parser)),
            4 => VerificationTypes::ItemLong(VerificationTypeLongVariable::new(parser)),
            5 => VerificationTypes::ItemNull(VerificationTypeNullVariable::new(parser)),
            6 => VerificationTypes::ItemUninitializedThis(
                VerificationTypeUninitializedThisVariable::new(parser),
            ),
            7 => VerificationTypes::ItemObject(VerificationTypeObjectVariable::new(parser)),
            8 => VerificationTypes::ItemUninitialized(VerificationTypeUninitializedVariable::new(
                parser,
            )),
            _ => panic!("{tag}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeTopVariable {
    tag: U1,
}

impl VerificationTypeTopVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeTopVariable {
        let tag = parser.consume_u1();

        VerificationTypeTopVariable { tag }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeIntegerVariable {
    tag: U1,
}

impl VerificationTypeIntegerVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeIntegerVariable {
        let tag = parser.consume_u1();

        VerificationTypeIntegerVariable { tag }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeFloatVariable {
    tag: U1,
}

impl VerificationTypeFloatVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeFloatVariable {
        let tag = parser.consume_u1();

        VerificationTypeFloatVariable { tag }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeLongVariable {
    tag: U1,
}

impl VerificationTypeLongVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeLongVariable {
        let tag = parser.consume_u1();

        VerificationTypeLongVariable { tag }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeDoubleVariable {
    tag: U1,
}

impl VerificationTypeDoubleVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeDoubleVariable {
        let tag = parser.consume_u1();

        VerificationTypeDoubleVariable { tag }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeNullVariable {
    tag: U1,
}

impl VerificationTypeNullVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeNullVariable {
        let tag = parser.consume_u1();

        VerificationTypeNullVariable { tag }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeUninitializedThisVariable {
    tag: U1,
}

impl VerificationTypeUninitializedThisVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeUninitializedThisVariable {
        let tag = parser.consume_u1();

        VerificationTypeUninitializedThisVariable { tag }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeObjectVariable {
    tag: U1,
    cpool_index: U2,
}

impl VerificationTypeObjectVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeObjectVariable {
        let tag = parser.consume_u1();
        let cpool_index = parser.consume_u2();

        VerificationTypeObjectVariable { tag, cpool_index }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationTypeUninitializedVariable {
    tag: U1,
    offset: U2,
}

impl VerificationTypeUninitializedVariable {
    pub fn new(parser: &mut Parser) -> VerificationTypeUninitializedVariable {
        let tag = parser.consume_u1();
        let offset = parser.consume_u2();

        VerificationTypeUninitializedVariable { tag, offset }
    }
}
