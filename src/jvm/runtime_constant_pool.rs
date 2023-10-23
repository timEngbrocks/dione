use std::collections::HashMap;

use crate::class_loader::constant_pool_info::{ConstantPool, ConstantPoolInfo};

use self::{sym_ref_class_or_interface::SymRefClassOrInterface, sym_ref_field_of_class_or_interface::SymRefFieldOfClassOrInterface, sym_ref_method_of_class::SymRefMethodOfClass, sym_ref_method_of_interface::SymRefMethodOfInterface, sym_ref_method_handle::SymRefMethodHandle, sym_ref_method_type::SymRefMethodType, string_constant::StringConstant, numeric_constant::NumericConstant, sym_ref_dynamically_computed_constant::SymRefDynamicallyComputedConstant, sym_ref_dynamically_computed_call_site::SymRefDynamicallyComputedCallSite};

pub mod sym_ref_class_or_interface;
pub mod sym_ref_field_of_class_or_interface;
pub mod sym_ref_method_of_class;
pub mod sym_ref_method_of_interface;
pub mod sym_ref_method_handle;
pub mod sym_ref_method_type;
pub mod sym_ref_dynamically_computed_constant;
pub mod sym_ref_dynamically_computed_call_site;
pub mod string_constant;
pub mod numeric_constant;

#[derive(Clone)]
pub enum RuntimeConstants {
	SymRefClassOrInterface(SymRefClassOrInterface),
	SymRefFieldOfClassOrInterface(SymRefFieldOfClassOrInterface),
	SymRefMethodOfClass(SymRefMethodOfClass),
	SymRefMethodOfInterface(SymRefMethodOfInterface),
	SymRefMethodHandle(SymRefMethodHandle),
	SymRefMethodType(SymRefMethodType),
	SymRefDynamicallyComputedConstant(SymRefDynamicallyComputedConstant),
	SymRefDynamicallyComputedCallSite(SymRefDynamicallyComputedCallSite),
	StringConstant(StringConstant),
	NumericConstant(NumericConstant),
}

pub trait Loadable {
	type Output;

	fn load(&self) -> Self::Output;
}

pub trait RuntimeConstant {
	fn resolve(index: u16, constant_pool: &ConstantPool) -> Self;
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
				7 => RuntimeConstants::SymRefClassOrInterface(SymRefClassOrInterface::resolve(i, constant_pool)),
				9 => RuntimeConstants::SymRefFieldOfClassOrInterface(SymRefFieldOfClassOrInterface::resolve(i, constant_pool)),
				10 => RuntimeConstants::SymRefMethodOfClass(SymRefMethodOfClass::resolve(i, constant_pool)),
				11 => RuntimeConstants::SymRefMethodOfInterface(SymRefMethodOfInterface::resolve(i, constant_pool)),
				15 => RuntimeConstants::SymRefMethodHandle(SymRefMethodHandle::resolve(i, constant_pool)),
				16 => RuntimeConstants::SymRefMethodType(SymRefMethodType::resolve(i, constant_pool)),
				17 => RuntimeConstants::SymRefDynamicallyComputedConstant(SymRefDynamicallyComputedConstant::resolve(i, constant_pool)),
				18 => RuntimeConstants::SymRefDynamicallyComputedCallSite(SymRefDynamicallyComputedCallSite::resolve(i, constant_pool)),
				8 => RuntimeConstants::StringConstant(StringConstant::resolve(i, constant_pool)),
				3 | 4 | 5 | 6 => RuntimeConstants::NumericConstant(NumericConstant::resolve(i, constant_pool)),
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