use std::fmt;

use nom::{error::ParseError, number::complete::{be_u16, be_u8}, IResult};

use crate::{U1, U2};

use super::METHODHANDLE;

#[derive(Debug)]
pub struct MethodHandle {
    pub tag: U1,
    pub reference_kind: U1,
    pub reference_index: U2,
}

impl fmt::Display for MethodHandle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag: {}, Reference Kind: {}, Reference Index: {}", self.tag, self.reference_kind, self.reference_index)?;
        Ok(())
    }
}

pub fn methodhandle_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], MethodHandle> {
    let (input, reference_kind) = be_u8::<&[u8], E>(input).expect("Failed to read 'reference_kind'");
    let (input, reference_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'reference_index'");

    Ok((input, MethodHandle {
        tag: METHODHANDLE,
        reference_kind,
        reference_index,
    }))
}
