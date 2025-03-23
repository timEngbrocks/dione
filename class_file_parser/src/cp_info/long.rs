use std::fmt;

use nom::{error::ParseError, number::complete::be_u32, IResult};

use crate::{U1, U4};

use super::LONG;

#[derive(Debug)]
pub struct Long {
    pub tag: U1,
    pub high_bytes: U4,
    pub low_bytes: U4,
}

impl fmt::Display for Long {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tag: {}, High Bytes: {}, Low Bytes: {}", self.tag, self.high_bytes, self.low_bytes)?;
        Ok(())
    }
}

pub fn long_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], Long> {
    let (input, high_bytes) = be_u32::<&[u8], E>(input).expect("Failed to read 'high_bytes'");
    let (input, low_bytes) = be_u32::<&[u8], E>(input).expect("Failed to read 'low_bytes'");

    Ok((input, Long {
        tag: LONG,
        high_bytes,
        low_bytes,
    }))
}
