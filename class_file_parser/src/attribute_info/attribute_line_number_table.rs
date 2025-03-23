use std::fmt;

use nom::{error::ParseError, multi::length_count, number::streaming::be_u16, IResult};

use crate::U2;



#[derive(Debug, Clone)]
pub struct AttributeLineNumberTable {
    line_number_table_length: U2,
    line_number_table: Vec<LineNumberTableEntry>,
}

impl fmt::Display for AttributeLineNumberTable {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line_number_table = self.line_number_table.iter().map(|x| format!("{x}")).collect::<Vec<String>>().join(", ");
        write!(f, "Line Number Table Length: {}, Line Number Table: [{}]", self.line_number_table_length, line_number_table)?;
        Ok(())
    }
}

pub fn line_number_table_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], AttributeLineNumberTable> {
    let (input, line_number_table) = length_count(be_u16, line_number_table_entry_parser::<'a, E>)(input).expect("Failed to read 'line_number_table_length' or 'line_number_table'");
	let line_number_table_length = line_number_table.len() as U2;

    Ok((input, AttributeLineNumberTable {
        line_number_table_length,
        line_number_table,
    }))
}

#[derive(Debug, Clone)]
pub struct LineNumberTableEntry {
    start_pc: U2,
    line_number: U2,
}

impl fmt::Display for LineNumberTableEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{Start PC: {}, Line Number: {}}}", self.start_pc, self.line_number)?;
        Ok(())
    }
}

pub fn line_number_table_entry_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], LineNumberTableEntry> {
    let (input, start_pc) = be_u16::<&[u8], E>(input).expect("Failed to read 'start_pc'");
    let (input, line_number) = be_u16::<&[u8], E>(input).expect("Failed to read 'line_number'");

    Ok((input, LineNumberTableEntry {
        start_pc,
        line_number,
    }))
}
