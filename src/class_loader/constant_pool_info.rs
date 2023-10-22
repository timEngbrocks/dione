use std::fmt;

use self::{constant_utf8_info::ConstantUtf8Info, constant_integer_info::ConstantIntegerInfo, constant_float_info::ConstantFloatInfo, constant_long_info::ConstantLongInfo, constant_double_info::ConstantDoubleInfo, constant_class_info::ConstantClassInfo, constant_string_info::ConstantStringInfo, constant_fieldref_info::ConstantFieldrefInfo, constant_methodref_info::ConstantMethodrefInfo, constant_interface_methodref_info::ConstantInterfaceMethodrefInfo, constant_name_and_type_info::ConstantNameAndTypeInfo, constant_method_handle_info::ConstantMethodHandleInfo, constant_method_type_info::ConstantMethodTypeInfo, constant_dynamic_info::ConstantDynamicInfo, constant_invoke_dynamic_info::ConstantInvokeDynamicInfo, constant_module_info::ConstantModuleInfo, constant_package_info::ConstantPackageInfo};

use super::parser::{U1, U2, Parser};

pub mod constant_class_info;
pub mod constant_fieldref_info;
pub mod constant_methodref_info;
pub mod constant_interface_methodref_info;
pub mod constant_string_info;
pub mod constant_integer_info;
pub mod constant_float_info;
pub mod constant_long_info;
pub mod constant_double_info;
pub mod constant_name_and_type_info;
pub mod constant_utf8_info;
pub mod constant_method_handle_info;
pub mod constant_method_type_info;
pub mod constant_dynamic_info;
pub mod constant_invoke_dynamic_info;
pub mod constant_module_info;
pub mod constant_package_info;

#[repr(u8)]
pub enum ConstantPoolInfoType {
	EmptyItem(ConstantEmptyItem) = 0,
	Utf8(ConstantUtf8Info) = 1,
    Integer(ConstantIntegerInfo) = 3,
    Float(ConstantFloatInfo) = 4,
    Long(ConstantLongInfo) = 5,
    Double(ConstantDoubleInfo) = 6,
    Class(ConstantClassInfo) = 7,
    String(ConstantStringInfo) = 8,
    Fieldref(ConstantFieldrefInfo) = 9,
    Methodref(ConstantMethodrefInfo) = 10,
    InterfaceMethodref(ConstantInterfaceMethodrefInfo) = 11,
    NameAndType(ConstantNameAndTypeInfo) = 12,
    MethodHandle(ConstantMethodHandleInfo) = 15,
    MethodType(ConstantMethodTypeInfo) = 16,
    Dynamic(ConstantDynamicInfo) = 17,
    InvokeDynamic(ConstantInvokeDynamicInfo) = 18,
    Module(ConstantModuleInfo) = 19,
    Package(ConstantPackageInfo) = 20,
}

impl ConstantPoolInfoType {
	pub fn tag_to_name(&self) -> &str {
		match self.get_tag() {
			0 => "[Empty Item]",
			1 => "Utf8",
			3 => "Integer",
			4 => "Float",
			5 => "Long",
			6 => "Double",
			7 => "Class",
			8 => "String",
			9 => "Fieldref",
			10 => "Methodref",
			11 => "InterfaceMethodref",
			12 => "NameAndType",
			15 => "MethodHandle",
			16 => "MethodType",
			17 => "Dynamic",
			18 => "InvokeDynamic",
			19 => "Module",
			20 => "Package",
			tag => panic!("Unknown constant tag: {tag}"),
		}
	}
}

pub trait ConstantPoolInfo {
	fn new(parser: &mut Parser) -> Self;
	fn get_tag(&self) -> &U1;
}

impl ConstantPoolInfo for ConstantPoolInfoType {
	fn new(parser: &mut Parser) -> Self {
		let tag = parser.peek_u1();
		match tag {
			1 => ConstantPoolInfoType::Utf8(ConstantUtf8Info::new(parser)),
			3 => ConstantPoolInfoType::Integer(ConstantIntegerInfo::new(parser)),
			4 => ConstantPoolInfoType::Float(ConstantFloatInfo::new(parser)),
			5 => ConstantPoolInfoType::Long(ConstantLongInfo::new(parser)),
			6 => ConstantPoolInfoType::Double(ConstantDoubleInfo::new(parser)),
			7 => ConstantPoolInfoType::Class(ConstantClassInfo::new(parser)),
			8 => ConstantPoolInfoType::String(ConstantStringInfo::new(parser)),
			9 => ConstantPoolInfoType::Fieldref(ConstantFieldrefInfo::new(parser)),
			10 => ConstantPoolInfoType::Methodref(ConstantMethodrefInfo::new(parser)),
			11 => ConstantPoolInfoType::InterfaceMethodref(ConstantInterfaceMethodrefInfo::new(parser)),
			12 => ConstantPoolInfoType::NameAndType(ConstantNameAndTypeInfo::new(parser)),
			15 => ConstantPoolInfoType::MethodHandle(ConstantMethodHandleInfo::new(parser)),
			16 => ConstantPoolInfoType::MethodType(ConstantMethodTypeInfo::new(parser)),
			17 => ConstantPoolInfoType::Dynamic(ConstantDynamicInfo::new(parser)),
			18 => ConstantPoolInfoType::InvokeDynamic(ConstantInvokeDynamicInfo::new(parser)),
			19 => ConstantPoolInfoType::Module(ConstantModuleInfo::new(parser)),
			20 => ConstantPoolInfoType::Package(ConstantPackageInfo::new(parser)),
			_ => panic!("Unknown constant tag: {tag}"),
		}
	}

	fn get_tag(&self) -> &U1 {
		match self {
			ConstantPoolInfoType::Class(x) => x.get_tag(),
			ConstantPoolInfoType::Fieldref(x) => x.get_tag(),
			ConstantPoolInfoType::Methodref(x) => x.get_tag(),
			ConstantPoolInfoType::InterfaceMethodref(x) => x.get_tag(),
			ConstantPoolInfoType::String(x) => x.get_tag(),
			ConstantPoolInfoType::Integer(x) => x.get_tag(),
			ConstantPoolInfoType::Float(x) => x.get_tag(),
			ConstantPoolInfoType::Long(x) => x.get_tag(),
			ConstantPoolInfoType::Double(x) => x.get_tag(),
			ConstantPoolInfoType::NameAndType(x) => x.get_tag(),
			ConstantPoolInfoType::Utf8(x) => x.get_tag(),
			ConstantPoolInfoType::MethodHandle(x) => x.get_tag(),
			ConstantPoolInfoType::MethodType(x) => x.get_tag(),
			ConstantPoolInfoType::Dynamic(x) => x.get_tag(),
			ConstantPoolInfoType::InvokeDynamic(x) => x.get_tag(),
			ConstantPoolInfoType::Module(x) => x.get_tag(),
			ConstantPoolInfoType::Package(x) => x.get_tag(),
			ConstantPoolInfoType::EmptyItem(x) => x.get_tag(),
		}	
	}
}

pub struct ConstantPool {
	constants: Vec<ConstantPoolInfoType>
}

impl fmt::Debug for ConstantPool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let tags: Vec<u8> = self.constants.iter().map::<u8, _>(|constant| constant.get_tag().clone()).collect();
		write!(f, "Constants: [{}]", itertools::join(&mut tags.iter(), ", "))
    }
}

impl ConstantPool {
	pub fn new(constants: Vec<ConstantPoolInfoType>) -> ConstantPool {
		ConstantPool { constants }
	}

	pub fn get(&self, index: U2) -> &ConstantPoolInfoType {
		match self.constants.get((index - 1) as usize) {
			Some(constant) => constant,
			None => panic!("{index}"),
		}
	}

	pub fn len(&self) -> U2 {
		self.constants.len() as U2
	}
}

pub struct ConstantEmptyItem {
	pub tag: U1,
}

impl ConstantPoolInfo for ConstantEmptyItem {
	fn new(_: &mut Parser) -> Self {
		let tag = 0;
		ConstantEmptyItem { tag }
	}

	fn get_tag(&self) -> &U1 {
		&self.tag
	}
}