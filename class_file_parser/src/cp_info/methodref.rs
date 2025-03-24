use std::fmt;

use nom::{IResult, error::ParseError, number::complete::be_u16};

use util::numbers::{U1, U2};

use super::METHODREF;

#[derive(Debug)]
pub struct Methodref {
    pub tag: U1,
    pub class_index: U2,
    pub name_and_type_index: U2,
}

impl fmt::Display for Methodref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tag: {}, Class Index: {}, Name and Type Index: {}",
            self.tag, self.class_index, self.name_and_type_index
        )?;
        Ok(())
    }
}

pub fn methodref_parser<'a, E: ParseError<&'a [u8]> + std::fmt::Debug>(
    input: &'a [u8],
) -> IResult<&'a [u8], Methodref> {
    let (input, class_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'class_index'");
    let (input, name_and_type_index) =
        be_u16::<&[u8], E>(input).expect("Failed to read 'name_and_type_index'");

    Ok((input, Methodref {
        tag: METHODREF,
        class_index,
        name_and_type_index,
    }))
}
