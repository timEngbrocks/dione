use std::collections::HashMap;

use crate::class_loader::constant_pool_info::{ConstantPool, ConstantPoolInfo};

use self::{runtime_constant_class::RuntimeConstantClass, runtime_constant_string::RuntimeConstantString, runtime_constant_integer::RuntimeConstantInteger, runtime_constant_long::RuntimeConstantLong, runtime_constant_float::RuntimeConstantFloat, runtime_constant_double::RuntimeConstantDouble};

pub mod runtime_constant_class;
pub mod runtime_constant_string;
pub mod runtime_constant_integer;
pub mod runtime_constant_long;
pub mod runtime_constant_float;
pub mod runtime_constant_double;

#[derive(Clone)]
pub enum RuntimeConstants {
	Class(RuntimeConstantClass),
	String(RuntimeConstantString),
	Integer(RuntimeConstantInteger),
	Long(RuntimeConstantLong),
	Float(RuntimeConstantFloat),
	Double(RuntimeConstantDouble),
}

pub struct RuntimeConstantPool {
	constants: HashMap<u16, RuntimeConstants>,
	length: u16,
}

impl RuntimeConstantPool {
	pub fn new(constant_pool: &ConstantPool) -> Self {
		let mut constants: HashMap<u16, RuntimeConstants> = HashMap::new();
		for i in 1..=constant_pool.len() {
			let constant = match constant_pool.get(i).get_tag() {
				// 1 => RuntimeConstants::Utf8(RuntimeConstantUtf8::resolve(i, constant_pool)),
				3 => RuntimeConstants::Integer(RuntimeConstantInteger::resolve(i, constant_pool)),
				4 => RuntimeConstants::Float(RuntimeConstantFloat::resolve(i, constant_pool)),
				5 => RuntimeConstants::Long(RuntimeConstantLong::resolve(i, constant_pool)),
				6 => RuntimeConstants::Double(RuntimeConstantDouble::resolve(i, constant_pool)),
				7 => RuntimeConstants::Class(RuntimeConstantClass::resolve(i, constant_pool)),
				8 => RuntimeConstants::String(RuntimeConstantString::resolve(i, constant_pool)),
				// 9 => RuntimeConstants::Fieldref(RuntimeConstantFieldref::resolve(i, constant_pool)),
				// 10 => RuntimeConstants::Methodref(RuntimeConstantMethodref::resolve(i, constant_pool)),
				// 11 => RuntimeConstants::InterfaceMethodref(RuntimeConstantInterfaceMethodref::resolve(i, constant_pool)),
				// 12 => RuntimeConstants::NameAndType(RuntimeConstantNameAndType::resolve(i, constant_pool)),
				// 15 => RuntimeConstants::MethodHandle(RuntimeConstantMethodHandle::resolve(i, constant_pool)),
				// 16 => RuntimeConstants::MethodType(RuntimeConstantMethodType::resolve(i, constant_pool)),
				// 17 => RuntimeConstants::Dynamic(RuntimeConstantDynamic::resolve(i, constant_pool)),
				// 18 => RuntimeConstants::InvokeDynamic(RuntimeConstantInvokeDynamic::resolve(i, constant_pool)),
				// 19 => RuntimeConstants::Module(RuntimeConstantModule::resolve(i, constant_pool)),
				// 20 => RuntimeConstants::Package(RuntimeConstantPackage::resolve(i, constant_pool)),
				_ => continue,
			};
			constants.insert(i as u16, constant);
		}

		RuntimeConstantPool {
			constants,
			length: constant_pool.len() as u16,
		}
	}

	pub fn clone(&self) -> RuntimeConstantPool {
		RuntimeConstantPool {
			constants: self.constants.clone(),
			length: self.length,
		}
	}

	pub fn get(&self, index: u16) -> &RuntimeConstants {
		match self.constants.get(&(index - 1)) {
			Some(constant) => constant,
			None => panic!("{index}"),
		}
	}

	pub fn len(&self) -> u16 {
		self.length
	}
}

pub trait RuntimeConstant {
	fn resolve(index: u16, constant_pool: &ConstantPool) -> Self;
}