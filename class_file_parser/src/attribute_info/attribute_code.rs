use std::fmt;

use nom::{error::ParseError, multi::length_count, number::streaming::{be_u16, be_u32, be_u8}, IResult};

use crate::{cp_info::CPInfo, U1, U2, U4};

use super::{attribute_info_parser, AttributeInfo};

#[derive(Debug)]
pub struct AttributeCode {
    max_stack: U2,
    max_locals: U2,
    code_length: U4,
    code: Vec<U1>,
    exception_table_length: U2,
    exception_table: Vec<ExceptionTableEntry>,
    attributes_count: U2,
    attributes: Vec<AttributeInfo>,
}

impl fmt::Display for AttributeCode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code = self.code.iter().map(|x| format!("{x:08b}")).collect::<Vec<String>>().join(" ");
        let exception_table = self.exception_table.iter().map(|x| format!("{x}")).collect::<Vec<String>>().join(", ");
        let attributes = self.attributes.iter().map(|x| format!("{x}")).collect::<Vec<String>>().join(", ");
        write!(f, "Max Stack: {}, Max Locals: {}, Code Length: {}, Code: {}, Exception Table Length: {}, Exception Table: [{}], Attributes Count: {}, Attributes: [{}]", self.max_stack, self.max_locals, self.code_length, code, self.exception_table_length, exception_table, self.attributes_count, attributes)?;
        Ok(())
    }
}

pub fn code_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8], constant_pool: &Vec<CPInfo>) -> IResult<&'a[u8], AttributeCode> {
    let (input, max_stack) = be_u16::<&[u8], E>(input).expect("Failed to read 'max_stack'");
    let (input, max_locals) = be_u16::<&[u8], E>(input).expect("Failed to read 'max_locals'");
    let (input, code) = length_count(be_u32, be_u8::<&[u8], E>)(input).expect("Failed to read 'code_length' or 'code'");
	let code_length = code.len() as U4;
    let (input, exception_table) = length_count(be_u16, exception_table_entry_parser::<'a, E>)(input).expect("Failed to read 'exception_table_length' or 'exception_table'");
	let exception_table_length = exception_table.len() as U2;
    let (input, attributes) = length_count(be_u16, |x| attribute_info_parser::<E>(x, constant_pool))(input).expect("Failed to read 'attributes_count' or 'attributes'");
	let attributes_count = attributes.len() as U2;

    Ok((input, AttributeCode {
        max_stack,
        max_locals,
        code_length,
        code,
        exception_table_length,
        exception_table,
        attributes_count,
        attributes,
    }))
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    start_pc: U2,
    end_pc: U2,
    handler_pc: U2,
    catch_type: U2,
}

impl fmt::Display for ExceptionTableEntry {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{Start PC: {}, End PC: {}, Handler PC: {}, Catch Type: {}}}", self.start_pc, self.end_pc, self.handler_pc, self.catch_type)?;
        Ok(())
    }
}

pub fn exception_table_entry_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], ExceptionTableEntry> {
    let (input, start_pc) = be_u16::<&[u8], E>(input).expect("Failed to read 'start_pc'");
    let (input, end_pc) = be_u16::<&[u8], E>(input).expect("Failed to read 'end_pc'");
    let (input, handler_pc) = be_u16::<&[u8], E>(input).expect("Failed to read 'handler_pc'");
    let (input, catch_type) = be_u16::<&[u8], E>(input).expect("Failed to read 'catch_type'");

    Ok((input, ExceptionTableEntry {
        start_pc,
        end_pc,
        handler_pc,
        catch_type
    }))
}