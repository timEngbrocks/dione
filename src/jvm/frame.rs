use crate::{
    class_loader::class_file::ClassFile,
    jvm::types::Types,
    util::{sized_array::SizedArray, stack::Stack},
};

use super::{runtime_constant_pool::RuntimeConstantPool, types::ReturnTypes};

pub struct Frame {
    local_variables: SizedArray<Types>,
    stack: Stack<Types>,
    runtime_constant_pool: RuntimeConstantPool,
    object_name: String,
    method_name: String,
    descriptor: String,
    return_value: ReturnTypes,
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
    ) -> Frame {
        Frame {
            local_variables,
            stack,
            runtime_constant_pool: RuntimeConstantPool::new(class_file),
            object_name,
            method_name,
            descriptor,
            return_value,
        }
    }

    pub fn assert_matches_return_type(&self, return_value: &Types) {
        match &self.return_value {
            ReturnTypes::Void => (),
            ReturnTypes::Type(t) => t.assert_matches_type(return_value),
        }
    }

    pub fn local_variables(&mut self) -> &mut SizedArray<Types> {
        &mut self.local_variables
    }

    pub fn stack(&mut self) -> &mut Stack<Types> {
        &mut self.stack
    }

    pub fn runtime_constant_pool(&self) -> &RuntimeConstantPool {
        &self.runtime_constant_pool
    }

    pub fn object_name(&self) -> &String {
        &self.object_name
    }

    pub fn method_name(&self) -> &String {
        &self.method_name
    }

    pub fn descriptor(&self) -> &String {
        &self.descriptor
    }

    pub fn return_value(&self) -> &ReturnTypes {
        &self.return_value
    }
}

impl Clone for Frame {
    fn clone(&self) -> Frame {
        Frame {
            local_variables: SizedArray::<Types>::new(self.local_variables.len()),
            stack: Stack::<Types>::new(self.stack.max_size()),
            runtime_constant_pool: self.runtime_constant_pool().clone(),
            object_name: self.object_name.clone(),
            method_name: self.method_name.clone(),
            descriptor: self.descriptor.clone(),
            return_value: self.return_value.clone(),
        }
    }
}
