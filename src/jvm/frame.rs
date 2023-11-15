use crate::{
    class_loader::class_file::ClassFile,
    jvm::types::Types,
    util::{sized_array::SizedArray, stack::Stack},
};

use super::{runtime_constant_pool::RuntimeConstantPool, types::{ReturnTypes, reference::Reference}};

pub struct Frame {
    pub local_variables: SizedArray<Types>,
    pub stack: Stack<Types>,
    pub runtime_constant_pool: RuntimeConstantPool,
    pub object_name: String,
    pub method_name: String,
    pub descriptor: String,
    pub return_value: ReturnTypes,
    pub object_ref: Option<Reference>,
}

impl Frame {
    pub fn new(
        local_variables: SizedArray<Types>,
        stack: Stack<Types>,
        class_file: &ClassFile,
        object_name: String,
        method_name: String,
        descriptor: String,
        return_value: ReturnTypes,
        object_ref: Option<Reference>,
    ) -> Frame {
        Frame {
            local_variables,
            stack,
            runtime_constant_pool: RuntimeConstantPool::new(class_file),
            object_name,
            method_name,
            descriptor,
            return_value,
            object_ref,
        }
    }

    pub fn new_native(class_file: &ClassFile, object_name: String, method_name: String, descriptor: String, return_value: ReturnTypes, object_ref: Option<Reference>) -> Frame {
        Frame {
            local_variables: SizedArray::<Types>::new(None),
            stack: Stack::<Types>::new(None),
            runtime_constant_pool: RuntimeConstantPool::new(class_file),
            object_name,
            method_name,
            descriptor,
            return_value,
            object_ref,
        }
    }

    pub fn assert_matches_return_type(&self, return_value: &Types) {
        match &self.return_value {
            ReturnTypes::Void => (),
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
            descriptor: self.descriptor.clone(),
            return_value: self.return_value.clone(),
            object_ref: self.object_ref.clone(),
        }
    }
}
