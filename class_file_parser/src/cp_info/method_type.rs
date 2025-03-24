use std::fmt;

use nom::{IResult, error::ParseError, number::complete::be_u16};

use util::numbers::{U1, U2};

use super::METHODTYPE;

#[derive(Debug)]
pub struct MethodType {
    pub tag: U1,
    pub descriptor_index: U2,
}

impl fmt::Display for MethodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tag: {}, Descriptor Index: {}",
            self.tag, self.descriptor_index
        )?;
        Ok(())
    }
}

pub fn methodtype_parser<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(
    input: &'a [u8],
) -> IResult<&'a [u8], MethodType> {
    let (input, descriptor_index) =
        be_u16::<&[u8], E>(input).expect("Failed to read 'descriptor_index'");

    Ok((input, MethodType {
        tag: METHODTYPE,
        descriptor_index,
    }))
}
