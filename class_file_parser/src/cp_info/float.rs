use std::fmt;

use nom::{IResult, error::ParseError, number::complete::be_u32};
use util::numbers::{U1, U4};

use super::FLOAT;

#[derive(Debug)]
pub struct Float {
    pub tag: U1,
    pub bytes: U4,
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag: {}, Bytes: {}", self.tag, self.bytes)?;
        Ok(())
    }
}

pub fn float_parser<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(
    input: &'a [u8],
) -> IResult<&'a [u8], Float> {
    let (input, bytes) = be_u32::<&[u8], E>(input).expect("Failed to read 'bytes'");

    Ok((input, Float { tag: FLOAT, bytes }))
}
