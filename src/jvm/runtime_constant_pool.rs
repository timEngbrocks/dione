use std::collections::HashMap;

use crate::class_loader::{constant_pool_info::ConstantPoolInfo, class_file::ClassFile};

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
	fn resolve(index: u16, class_file: &ClassFile) -> Self;
}

pub struct RuntimeConstantPool {
	constants: HashMap<u16, RuntimeConstants>,
	length: u16,
}

impl RuntimeConstantPool {
	pub fn new(class_file: &ClassFile) -> Self {
		let mut constants: HashMap<u16, RuntimeConstants> = HashMap::new();
		for i in 1..=class_file.constant_pool.len() {
			let constant = match RuntimeConstantPool::resolve(i, class_file) {
				Some(constant) => constant,
				None => continue,
			};
			constants.insert(i, constant);
		}

		RuntimeConstantPool {
			constants,
			length: class_file.constant_pool.len(),
		}
	}

	pub fn resolve(index: u16, class_file: &ClassFile) -> Option<RuntimeConstants> {
		match class_file.constant_pool.get(index).get_tag() {
			7 => Some(RuntimeConstants::SymRefClassOrInterface(SymRefClassOrInterface::resolve(index, class_file))),
			9 => Some(RuntimeConstants::SymRefFieldOfClassOrInterface(SymRefFieldOfClassOrInterface::resolve(index, class_file))),
			10 => Some(RuntimeConstants::SymRefMethodOfClass(SymRefMethodOfClass::resolve(index, class_file))),
			11 => Some(RuntimeConstants::SymRefMethodOfInterface(SymRefMethodOfInterface::resolve(index, class_file))),
			15 => Some(RuntimeConstants::SymRefMethodHandle(SymRefMethodHandle::resolve(index, class_file))),
			16 => Some(RuntimeConstants::SymRefMethodType(SymRefMethodType::resolve(index, class_file))),
			17 => Some(RuntimeConstants::SymRefDynamicallyComputedConstant(SymRefDynamicallyComputedConstant::resolve(index, class_file))),
			18 => Some(RuntimeConstants::SymRefDynamicallyComputedCallSite(SymRefDynamicallyComputedCallSite::resolve(index, class_file))),
			8 => Some(RuntimeConstants::StringConstant(StringConstant::resolve(index, class_file))),
			3..=6 => Some(RuntimeConstants::NumericConstant(NumericConstant::resolve(index, class_file))),
			_ => None,
		}
	}

	pub fn get(&self, index: u16) -> &RuntimeConstants {
		match self.constants.get(&index) {
			Some(constant) => constant,
			None => panic!("{index}"),
		}
	}

	pub fn len(&self) -> u16 {
		self.length
	}
}

impl Clone for RuntimeConstantPool {
	fn clone(&self) -> RuntimeConstantPool {
		RuntimeConstantPool {
			constants: self.constants.clone(),
			length: self.length,
		}
	}
}