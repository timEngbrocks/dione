use std::cmp::Ordering;

use crate::{class_loader::{constant_pool_info::{ConstantPoolInfoType, constant_method_handle_info::ConstantMethodHandleKind}, class_file::{compare_class_file_version_to_global, ClassFile}}, resolve_constant};

use super::{RuntimeConstant, sym_ref_field_of_class_or_interface::SymRefFieldOfClassOrInterface, sym_ref_method_of_class::SymRefMethodOfClass, sym_ref_method_of_interface::SymRefMethodOfInterface};

#[derive(Clone)]
pub enum MethodHandleKind {
	FieldRef(SymRefFieldOfClassOrInterface),
	ClassMethod(SymRefMethodOfClass),
	InterfaceMethod(SymRefMethodOfInterface),
}

#[derive(Clone)]
pub struct SymRefMethodHandle {
	pub kind: MethodHandleKind,
}

impl RuntimeConstant for SymRefMethodHandle {
	fn resolve(index: u16, class_file: &ClassFile) -> Self {
		let method_handle = resolve_constant!(ConstantPoolInfoType::MethodHandle, index, class_file.constant_pool);
		let kind = ConstantMethodHandleKind::from_kind(method_handle.reference_kind);

		match kind {
			ConstantMethodHandleKind::GetField |
			ConstantMethodHandleKind::GetStatic |
			ConstantMethodHandleKind::PutField |
			ConstantMethodHandleKind::PutStatic => {
				// CONSTANT_Fieldref_info
				let field_ref = SymRefFieldOfClassOrInterface::resolve(method_handle.reference_index, class_file);
				SymRefMethodHandle {
					kind: MethodHandleKind::FieldRef(field_ref),
				}
			},
			ConstantMethodHandleKind::InvokeVirtual |
			ConstantMethodHandleKind::NewInvokeSpecial => {
				// CONSTANT_Methodref_info
				let method_ref = SymRefMethodOfClass::resolve(method_handle.reference_index, class_file);
				SymRefMethodHandle {
					kind: MethodHandleKind::ClassMethod(method_ref),
				}
			},
			ConstantMethodHandleKind::InvokeStatic |
			ConstantMethodHandleKind::InvokeSpecial => {
				// Class File Version < 52.0: CONSTANT_Methodref_info 
				// Else: CONSTANT_Methodref_info or CONSTANT_InterfaceMethodref_info
				if compare_class_file_version_to_global(52, 0) == Ordering::Less {
					let method_ref = SymRefMethodOfClass::resolve(method_handle.reference_index, class_file);
					SymRefMethodHandle {
						kind: MethodHandleKind::ClassMethod(method_ref),
					}
				} else {
					match class_file.constant_pool.get(index) {
						ConstantPoolInfoType::Methodref(_) => {
							let method_ref = SymRefMethodOfClass::resolve(method_handle.reference_index, class_file);
							SymRefMethodHandle {
								kind: MethodHandleKind::ClassMethod(method_ref),
							}
						},
						ConstantPoolInfoType::InterfaceMethodref(_) => {
							let method_ref = SymRefMethodOfInterface::resolve(method_handle.reference_index, class_file);
							SymRefMethodHandle {
								kind: MethodHandleKind::InterfaceMethod(method_ref),
							}
						},
						_ => panic!("Invalid constant pool item at index, should be MethodRef or InterfaceMethodRef: {}", index),
					}
				}
			},
			ConstantMethodHandleKind::InvokeInterface => {
				// CONSTANT_InterfaceMethodref_info
				let method_ref = SymRefMethodOfInterface::resolve(method_handle.reference_index, class_file);
				SymRefMethodHandle {
					kind: MethodHandleKind::InterfaceMethod(method_ref),
				}
			},
		}
	}
}

fn is_allowed_method_name(name: &str, kind: ConstantMethodHandleKind) -> bool {
	match kind {
		ConstantMethodHandleKind::InvokeVirtual |
		ConstantMethodHandleKind::InvokeStatic |
		ConstantMethodHandleKind::InvokeSpecial |
		ConstantMethodHandleKind::InvokeInterface => {
			name != "<init>" && name != "<clinit>"
		},
		ConstantMethodHandleKind::NewInvokeSpecial => {
			name == "<init>"
		},
		_ => {
			true
		}
	}
}

// FIXME: impl Loadable for SymRefMethodHandle