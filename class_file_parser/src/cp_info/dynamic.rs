use std::fmt;

use nom::{error::ParseError, number::complete::be_u16, IResult};

use crate::{U1, U2};

use super::DYNAMIC;

#[derive(Debug)]
pub struct Dynamic {
    pub tag: U1,
    pub bootstrap_method_attr_index: U2,
    pub name_and_type_index: U2,
}

impl fmt::Display for Dynamic {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag: {}, Bootstrap Method Attr Index: {}, Name and Type Index: {}", self.tag, self.bootstrap_method_attr_index, self.name_and_type_index)?;
        Ok(())
    }
}

pub fn dynamic_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], Dynamic> {
    let (input, bootstrap_method_attr_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'bootstrap_method_attr_index'");
    let (input, name_and_type_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_and_type_index'");

    Ok((input, Dynamic {
        tag: DYNAMIC,
        bootstrap_method_attr_index,
        name_and_type_index,
    }))
}
