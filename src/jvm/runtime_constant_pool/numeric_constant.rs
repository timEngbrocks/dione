use crate::{
    class_loader::{class_file::ClassFile, constant_pool_info::ConstantPoolInfoType},
    jvm::types::{double::Double, float::Float, int::Int, long::Long, Value},
};

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
    value: NumericConstantKind,
}
impl NumericConstant {
    pub fn value(&self) -> &NumericConstantKind {
        &self.value
    }
}
impl RuntimeConstant for NumericConstant {
    fn resolve(index: u16, class_file: &ClassFile) -> Self {
        match class_file.constant_pool().get(&index) {
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
