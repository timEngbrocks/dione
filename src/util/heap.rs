use std::{rc::Rc, cell::RefCell};

use crate::jvm::types::{Value, reference::Reference, ReferenceableTypes};

use super::allocator::JVMAllocator;

pub enum ReferencePtr {
	Null,
	Class(ClassPtr),
	Array(ArrayPtr),
	Interface(InterfacePtr),
}

impl ReferencePtr {
	pub fn clone(&self) -> ReferencePtr {
		match self {
			ReferencePtr::Null => ReferencePtr::Null,
			ReferencePtr::Class(value) => ReferencePtr::Class(Rc::clone(value)),
			ReferencePtr::Array(value) => ReferencePtr::Array(Rc::clone(value)),
			ReferencePtr::Interface(value) => ReferencePtr::Interface(Rc::clone(value)),
		}
	}
}


static CLASS_ALLOCATOR: JVMAllocator = JVMAllocator {};
static ARRAY_ALLOCATOR: JVMAllocator = JVMAllocator {};
static INTERFACE_ALLOCATOR: JVMAllocator = JVMAllocator {};

pub type ClassPtr = Rc<RefCell<ReferenceableTypes>, &'static JVMAllocator>;
pub type ArrayPtr = Rc<RefCell<ReferenceableTypes>, &'static JVMAllocator>;
pub type InterfacePtr = Rc<RefCell<ReferenceableTypes>, &'static JVMAllocator>;

static mut INSTANCE: Option<Heap> = None;

const HEAP_START: usize = 0x10000000; // 256 MB
const HEAP_SIZE: usize = 1024 * 1024; // 1 MB

pub struct Heap {}

impl Heap {
	pub fn initialize() {
		Self::it();
	}

	pub fn allocate(value: ReferenceableTypes) -> Reference {
		Self::it().allocate_impl(value)
	}

	fn it() -> &'static Self {
		unsafe {
			if let Some(_) = INSTANCE {
				INSTANCE.as_ref().unwrap()
			} else {
				INSTANCE = Some(Heap::new());
				INSTANCE.as_ref().unwrap()
			}
		}
	}

	fn new() -> Self {
		Heap {}
	}

	fn allocate_impl(&self, value: ReferenceableTypes) -> Reference {
		match value {
			ReferenceableTypes::Class(_) => {
				let value = Rc::new_in(RefCell::new(value), &CLASS_ALLOCATOR);
				Reference::from_value(ReferencePtr::Class(value))
			},
			ReferenceableTypes::Array(_) => {
				let value = Rc::new_in(RefCell::new(value), &ARRAY_ALLOCATOR);
				Reference::from_value(ReferencePtr::Array(value))
			},
			ReferenceableTypes::Interface(_) => {
				let value = Rc::new_in(RefCell::new(value), &INTERFACE_ALLOCATOR);
				Reference::from_value(ReferencePtr::Interface(value))
			},
		}
	}
}
