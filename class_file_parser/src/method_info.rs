use std::fmt;

use nom::{error::ParseError, multi::length_count, number::complete::be_u16, IResult};

use crate::{attribute_info::{attribute_info_parser, AttributeInfo}, cp_info::CPInfo, U2};

#[derive(Debug)]
pub struct MethodInfo {
	pub access_flags: U2,
	pub name_index: U2,
	pub descriptor_index: U2,
	pub attributes_count: U2,
	pub attributes: Vec<AttributeInfo>,
}

impl fmt::Display for MethodInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(f, "Access Flags: {:#016b}, Name Index: {}, Descriptor Index: {}, Attributes Count: {}", self.access_flags, self.name_index, self.descriptor_index, self.attributes_count)?;
		if self.attributes_count > 0 {
			writeln!(f, "\t\tAttributes:")?;
		}
		for (i, attribute) in self.attributes[..self.attributes.len() - 1].iter().enumerate() {
			writeln!(f, "\t\t[{i}]: {}", attribute)?;
		}
		write!(f, "\t\t[{}]: {}", self.attributes_count - 1, self.attributes[self.attributes.len() - 1])?;
		Ok(())
	}
}

pub fn method_info_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8], constant_pool: &Vec<CPInfo>) -> IResult<&'a[u8], MethodInfo> {
	let (input, access_flags) = be_u16::<&[u8], E>(input).expect("Failed to read 'access_flags'");
	let (input, name_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'name_index'");
	let (input, descriptor_index) = be_u16::<&[u8], E>(input).expect("Failed to read 'descriptor_index'");
	
	let (input, attributes) = length_count(be_u16, |x| attribute_info_parser::<E>(x, constant_pool))(input).expect("Failed to read 'attributes_count' or 'attributes'");
	let attributes_count = attributes.len() as U2;

	Ok((input, MethodInfo {
		access_flags,
		name_index,
		descriptor_index,
		attributes_count,
		attributes,
	}))
}