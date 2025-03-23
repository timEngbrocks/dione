use std::fmt;

use nom::{error::ParseError, number::streaming::be_u16, IResult};

use crate::U2;

#[derive(Debug, Clone)]
pub struct AttributeSourceFile {
    source_file_index: U2,
}

impl fmt::Display for AttributeSourceFile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source File Index: {}", self.source_file_index)?;
        Ok(())
    }
}

pub fn source_file_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], AttributeSourceFile> {
    let (input, source_file_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'source_file_index'");

    Ok((input, AttributeSourceFile {
        source_file_index,
    }))
}
