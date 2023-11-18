use crate::class_loader::class_file::ClassFile;

use super::{sym_ref_method_handle::SymRefMethodHandle, RuntimeConstant, RuntimeConstants};

#[derive(Clone)]
pub struct SymRefDynamicallyComputedConstant {
    method_handle_ref: SymRefMethodHandle,
    arguments: Vec<RuntimeConstants>,
    name: String,
    descriptor: String,
}

impl RuntimeConstant for SymRefDynamicallyComputedConstant {
    fn resolve(_: u16, _: &ClassFile) -> Self {
        unimplemented!("SymRefDynamicallyComputedConstant::resolve")
    }
}

// FIXME: impl Loadable for SymRefDynamicallyComputedConstant
