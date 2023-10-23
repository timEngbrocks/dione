use crate::{class_loader::constant_pool_info::{ConstantPool, ConstantPoolInfoType}, jvm::types::{int::Int, float::Float, long::Long, double::Double, Value}};

use super::RuntimeConstant;

#[derive(Clone)]
pub enum NumericConstantKind {
	Integer(Int),
	Float(Float),
	Long(Long),
	Double(Double),
}

#[derive(Clone)]
pub struct NumericConstant {
	pub value: NumericConstantKind,
}

impl RuntimeConstant for NumericConstant {
	fn resolve(index: u16, constant_pool: &ConstantPool) -> Self {
		match constant_pool.get(index) {
			ConstantPoolInfoType::Integer(value) => {
				NumericConstant {
					value: NumericConstantKind::Integer(Int::from_value(value.to_i32())),
				}
			},
			ConstantPoolInfoType::Float(value) => {
				NumericConstant {
					value: NumericConstantKind::Float(Float::from_value(value.to_f32())),
				}
			},
			ConstantPoolInfoType::Long(value) => {
				NumericConstant {
					value: NumericConstantKind::Long(Long::from_value(value.to_i64())),
				}
			},
			ConstantPoolInfoType::Double(value) => {
				NumericConstant {
					value: NumericConstantKind::Double(Double::from_value(value.to_f64())),
				}
			},
			_ => panic!("NumericConstant::resolve: invalid constant pool type. Should be one of int/float/long/double"),
		}
	}
}

// FIXME: impl Loadable for NumericConstant