use crate::class_loader::constant_pool_info::ConstantPool;

use super::{RuntimeConstant, sym_ref_method_handle::SymRefMethodHandle, RuntimeConstants};

#[derive(Clone)]
pub struct SymRefDynamicallyComputedCallSite {
	pub method_handle_ref: SymRefMethodHandle,
	pub arguments: Vec<RuntimeConstants>,
	pub name: String,
	pub descriptor: String,
}

impl RuntimeConstant for SymRefDynamicallyComputedCallSite {
	fn resolve(_: u16, _: &ConstantPool) -> Self {
		unimplemented!("SymRefDynamicallyComputedCallSite::resolve")
	}
}

// FIXME: impl Loadable for SymRefDynamicallyComputedCallSite