use crate::{
    class_loader::class_file::ClassFile,
    jvm::types::Types,
    util::{sized_array::SizedArray, stack::Stack},
};

use super::{runtime_constant_pool::RuntimeConstantPool, types::ReturnTypes};

pub struct Frame {
    pub local_variables: SizedArray<Types>,
    pub stack: Stack<Types>,
    pub runtime_constant_pool: RuntimeConstantPool,
    pub object_name: String,
    pub method_name: String,
    pub return_value: ReturnTypes,
}

impl Frame {
    pub fn new(
        local_variables: SizedArray<Types>,
        stack: Stack<Types>,
        class_file: &ClassFile,
        object_name: String,
        method_name: String,
        return_value: ReturnTypes,
    ) -> Frame {
        Frame {
            local_variables,
            stack,
            runtime_constant_pool: RuntimeConstantPool::new(class_file),
            object_name,
            method_name,
            return_value,
        }
    }

    pub fn assert_matches_return_type(&self, return_value: &Types) {
        match &self.return_value {
            ReturnTypes::Void => panic!("Expected void return type"),
            ReturnTypes::Type(t) => t.assert_matches_type(return_value),
        }
    }
}

impl Clone for Frame {
    fn clone(&self) -> Frame {
        Frame {
            local_variables: SizedArray::<Types>::new(self.local_variables.len()),
            stack: Stack::<Types>::new(self.stack.max_size()),
            runtime_constant_pool: self.runtime_constant_pool.clone(),
            object_name: self.object_name.clone(),
            method_name: self.method_name.clone(),
            return_value: self.return_value.clone(),
        }
    }
}
