use std::fmt;

use class::{class_parser, Class};
use double::{double_parser, Double};
use dynamic::{dynamic_parser, Dynamic};
use fieldref::{fieldref_parser, Fieldref};
use float::{float_parser, Float};
use integer::{integer_parser, Integer};
use interface_methodref::{interfacemethodref_parser, InterfaceMethodref};
use invoke_dynamic::{invokedynamic_parser, InvokeDynamic};
use long::{long_parser, Long};
use method_handle::{methodhandle_parser, MethodHandle};
use method_type::{methodtype_parser, MethodType};
use methodref::{methodref_parser, Methodref};
use module::{module_parser, Module};
use name_and_type::{nameandtype_parser, NameAndType};
use nom::{error::ParseError, number::complete::be_u8, IResult};
use package::{package_parser, Package};
use string::string_parser;
use utf8::{utf8_parser, Utf8};

use crate::U1;

pub mod class;
pub mod double;
pub mod dynamic;
pub mod fieldref;
pub mod float;
pub mod integer;
pub mod interface_methodref;
pub mod invoke_dynamic;
pub mod long;
pub mod method_handle;
pub mod method_type;
pub mod methodref;
pub mod module;
pub mod name_and_type;
pub mod package;
pub mod string;
pub mod utf8;

#[derive(Debug)]
pub enum CPInfo {
	Class(Class),
	Fieldref(Fieldref),
	Methodref(Methodref),
	InterfaceMethodref(InterfaceMethodref),
	String(string::String),
	Integer(Integer),
	Float(Float),
	Long(Long),
	Double(Double),
	NameAndType(NameAndType),
	Utf8(Utf8),
	MethodHandle(MethodHandle),
	MethodType(MethodType),
	Dynamic(Dynamic),
	InvokeDynamic(InvokeDynamic),
	Module(Module),
	Package(Package),
}

impl fmt::Display for CPInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			CPInfo::Class(x) => write!(f, "Class: {x}")?,
			CPInfo::Fieldref(x) => write!(f, "Fieldref: {x}")?,
			CPInfo::Methodref(x) => write!(f, "Methodref: {x}")?,
			CPInfo::InterfaceMethodref(x) => write!(f, "InterfaceMethodref: {x}")?,
			CPInfo::String(x) => write!(f, "String: {x}")?,
			CPInfo::Integer(x) => write!(f, "Integer: {x}")?,
			CPInfo::Float(x) => write!(f, "Float: {x}")?,
			CPInfo::Long(x) => write!(f, "Long: {x}")?,
			CPInfo::Double(x) => write!(f, "Double: {x}")?,
			CPInfo::NameAndType(x) => write!(f, "NameAndType: {x}")?,
			CPInfo::Utf8(x) => write!(f, "Utf8: {x}")?,
			CPInfo::MethodHandle(x) => write!(f, "MethodHandle: {x}")?,
			CPInfo::MethodType(x) => write!(f, "MethodType: {x}")?,
			CPInfo::Dynamic(x) => write!(f, "Dynamic: {x}")?,
			CPInfo::InvokeDynamic(x) => write!(f, "InvokeDynamic: {x}")?,
			CPInfo::Module(x) => write!(f, "Module: {x}")?,
			CPInfo::Package(x) => write!(f, "Package: {x}")?,
		};
		Ok(())
	}
}

const CLASS: U1 = 7;
const FIELDREF: U1 = 9;
const METHODREF: U1 = 10;
const INTERFACEMETHODREF: U1 = 11;
const STRING: U1 = 8;
const INTEGER: U1 = 3;
const FLOAT: U1 = 4;
const LONG: U1 = 5;
const DOUBLE: U1 = 6;
const NAMEANDTYPE: U1 = 12;
const UTF8: U1 = 1;
const METHODHANDLE: U1 = 15;
const METHODTYPE: U1 = 16;
const DYNAMIC: U1 = 17;
const INVOKEDYNAMIC: U1 = 18;
const MODULE: U1 = 19;
const PACKAGE: U1 = 20;

pub fn cp_info_parser<'a, E: ParseError<&'a[u8]> + std::fmt::Debug>(input: &'a[u8]) -> IResult<&'a[u8], CPInfo> {
	let (input, tag) = be_u8::<&[u8], E>(input).expect("Failed to read 'tag'");
	
	match tag {
		CLASS => {
			let (input, value) = class_parser::<'a, E>(input).expect("Failed to read 'CLASS'");
			Ok((input, CPInfo::Class(value)))
		},
		FIELDREF => {
			let (input, value) = fieldref_parser::<'a, E>(input).expect("Failed to read 'FIELDREF'");
			Ok((input, CPInfo::Fieldref(value)))
		},
		METHODREF => {
			let (input, value) = methodref_parser::<'a, E>(input).expect("Failed to read 'METHODREF'");
			Ok((input, CPInfo::Methodref(value)))
		},
		INTERFACEMETHODREF => {
			let (input, value) = interfacemethodref_parser::<'a, E>(input).expect("Failed to read 'INTERFACEMETHODREF'");
			Ok((input, CPInfo::InterfaceMethodref(value)))
		},
		STRING => {
			let (input, value) = string_parser::<'a, E>(input).expect("Failed to read 'STRING'");
			Ok((input, CPInfo::String(value)))
		},
		INTEGER => {
			let (input, value) = integer_parser::<'a, E>(input).expect("Failed to read 'INTEGER'");
			Ok((input, CPInfo::Integer(value)))
		},
		FLOAT => {
			let (input, value) = float_parser::<'a, E>(input).expect("Failed to read 'FLOAT'");
			Ok((input, CPInfo::Float(value)))
		},
		LONG => {
			let (input, value) = long_parser::<'a, E>(input).expect("Failed to read 'LONG'");
			Ok((input, CPInfo::Long(value)))
		},
		DOUBLE => {
			let (input, value) = double_parser::<'a, E>(input).expect("Failed to read 'DOUBLE'");
			Ok((input, CPInfo::Double(value)))
		},
		NAMEANDTYPE => {
			let (input, value) = nameandtype_parser::<'a, E>(input).expect("Failed to read 'NAMEANDTYPE'");
			Ok((input, CPInfo::NameAndType(value)))
		},
		UTF8 => {
			let (input, value) = utf8_parser::<'a, E>(input).expect("Failed to read 'UTF8'");
			Ok((input, CPInfo::Utf8(value)))
		},
		METHODHANDLE => {
			let (input, value) = methodhandle_parser::<'a, E>(input).expect("Failed to read 'METHODHANDLE'");
			Ok((input, CPInfo::MethodHandle(value)))
		},
		METHODTYPE => {
			let (input, value) = methodtype_parser::<'a, E>(input).expect("Failed to read 'METHODTYPE'");
			Ok((input, CPInfo::MethodType(value)))
		},
		DYNAMIC => {
			let (input, value) = dynamic_parser::<'a, E>(input).expect("Failed to read 'DYNAMIC'");
			Ok((input, CPInfo::Dynamic(value)))
		},
		INVOKEDYNAMIC => {
			let (input, value) = invokedynamic_parser::<'a, E>(input).expect("Failed to read 'INVOKEDYNAMIC'");
			Ok((input, CPInfo::InvokeDynamic(value)))
		},
		MODULE => {
			let (input, value) = module_parser::<'a, E>(input).expect("Failed to read 'MODULE'");
			Ok((input, CPInfo::Module(value)))
		},
		PACKAGE => {
			let (input, value) = package_parser::<'a, E>(input).expect("Failed to read 'PACKAGE'");
			Ok((input, CPInfo::Package(value)))
		},
		v => panic!("Unknown constant tag: '{v}'"),
	}
}