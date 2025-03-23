use std::fmt;

use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::INVOKEDYNAMIC;

#[derive(Debug)]
pub struct InvokeDynamic {
    pub tag: U1,
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

impl fmt::Display for InvokeDynamic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag: {}, Bootstrap Method Attr Index: {}, Name and Type Index: {}", self.tag, self.bootstrap_method_attr_index, self.name_and_type_index)?;
        Ok(())
    }
}

pub fn invokedynamic_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], InvokeDynamic> {
    let (input, bootstrap_method_attr_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'bootstrap_method_attr_index'");
    let (input, name_and_type_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_and_type_index'");

    Ok((input, InvokeDynamic {
        tag: INVOKEDYNAMIC,
        bootstrap_method_attr_index,
        name_and_type_index,
    }))
}
