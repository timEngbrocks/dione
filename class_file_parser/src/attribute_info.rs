use std::fmt;

use attribute_code::{code_parser, AttributeCode};
use attribute_line_number_table::{line_number_table_parser, AttributeLineNumberTable};
use attribute_source_file::{source_file_parser, AttributeSourceFile};
use nom::{error::ParseError, number::streaming::{be_u16, be_u32}, IResult};

use crate::cp_info::CPInfo;

pub mod attribute_code;
pub mod attribute_line_number_table;
pub mod attribute_source_file;

#[derive(Debug)]
pub enum AttributeInfo {
	Code(AttributeCode),
	LineNumberTable(AttributeLineNumberTable),
	SourceFile(AttributeSourceFile),
}

impl fmt::Display for AttributeInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			AttributeInfo::Code(x) => write!(f, "Code: {x}")?,
			AttributeInfo::LineNumberTable(x) => write!(f, "LineNumberTable: {x}")?,
			AttributeInfo::SourceFile(x) => write!(f, "SourceFile: {x}")?,
		}
		Ok(())
	}
}

pub fn attribute_info_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8], constant_pool: &Vec<CPInfo>) -> IResult<&'a[u8], AttributeInfo> {
	let (input, attribute_name_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'attribute_name_index'");
	let attribute_name = match constant_pool.get((attribute_name_index - 1) as usize) {
		Some(CPInfo::Utf8(info)) => match String::from_utf8(info.bytes.clone()) {
            Ok(s) => s,
            Err(e) => panic!("Utf8 constant contained invalid bytes! Got: '{}'", e)
        },
		_ => panic!("Invalid attribute name index: {attribute_name_index}"),
	};
	let (input, _attribute_length) = be_u32::<&[u8], E>(input).expect("Failed to read 'attribute_length'");
	match attribute_name.as_str() {
		"Code" => {
			let (input, value) = code_parser::<E>(input, constant_pool).expect("Failed to parse 'code'");
			Ok((input, AttributeInfo::Code(value)))
		}
		"LineNumberTable" => {
			let (input, value) = line_number_table_parser::<'a, E>(input).expect("Failed to parse 'line_number_table'");
			Ok((input, AttributeInfo::LineNumberTable(value)))
		}
		"SourceFile" => {
			let (input, value) = source_file_parser::<'a, E>(input).expect("Failed to parse 'source_file'");
			Ok((input, AttributeInfo::SourceFile(value)))
		}
		name => panic!("Unimplemented attribute: '{name}'"),
	}
}