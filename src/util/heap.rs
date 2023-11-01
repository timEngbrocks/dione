use std::{rc::Rc, cell::RefCell};

use crate::jvm::types::{Value, reference::Reference, object::Object, array::{PrimitiveArray, ReferenceArray}};

use super::allocator::JVMAllocator;

#[derive(Clone)]
pub enum ReferencePtr {
	Null,
	Class(ClassPtr),
	Array(ArrayPtr),
	Interface(InterfacePtr),
}

#[derive(Clone)]
pub enum ArrayPtr {
	Primitive(PrimitiveArrayPtr),
	Reference(ReferenceArrayPtr),
}

impl ReferencePtr {
	pub fn clone(&self) -> ReferencePtr {
		match self {
			ReferencePtr::Null => ReferencePtr::Null,
			ReferencePtr::Class(value) => ReferencePtr::Class(Rc::clone(value)),
			ReferencePtr::Array(value) => match value {
				ArrayPtr::Primitive(value) => ReferencePtr::Array(ArrayPtr::Primitive(Rc::clone(value))),
				ArrayPtr::Reference(value) => ReferencePtr::Array(ArrayPtr::Reference(Rc::clone(value))),
			},
			ReferencePtr::Interface(value) => ReferencePtr::Interface(Rc::clone(value)),
		}
	}
}


static CLASS_ALLOCATOR: JVMAllocator = JVMAllocator {};
static PRIMITIVE_ARRAY_ALLOCATOR: JVMAllocator = JVMAllocator {};
static REFERENCE_ARRAY_ALLOCATOR: JVMAllocator = JVMAllocator {};
static INTERFACE_ALLOCATOR: JVMAllocator = JVMAllocator {};

pub type ClassPtr = Rc<RefCell<Object>, &'static JVMAllocator>;
pub type PrimitiveArrayPtr = Rc<RefCell<PrimitiveArray>, &'static JVMAllocator>;
pub type ReferenceArrayPtr = Rc<RefCell<ReferenceArray>, &'static JVMAllocator>;
pub type InterfacePtr = Rc<RefCell<Object>, &'static JVMAllocator>;

static mut INSTANCE: Option<Heap> = None;

pub struct Heap {}

impl Heap {
	pub fn initialize() {
		Self::it();
	}

	fn new() -> Self {
		Heap {}
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

	pub fn allocate_class(object: Object) -> Reference {
		Heap::it().allocate_class_impl(object)
	}

	fn allocate_class_impl(&self, object: Object) -> Reference {
		let value = Rc::new_in(RefCell::new(object), &CLASS_ALLOCATOR);
		Reference::from_value(ReferencePtr::Class(value))
	}

	pub fn allocate_primitive_array(array: PrimitiveArray) -> Reference {
		Heap::it().allocate_primitive_array_impl(array)
	}

	fn allocate_primitive_array_impl(&self, array: PrimitiveArray) -> Reference {
		let value = Rc::new_in(RefCell::new(array), &PRIMITIVE_ARRAY_ALLOCATOR);
		Reference::from_value(ReferencePtr::Array(ArrayPtr::Primitive(value)))
	}

	pub fn allocate_reference_array(array: ReferenceArray) -> Reference {
		Heap::it().allocate_reference_array_impl(array)
	}

	fn allocate_reference_array_impl(&self, array: ReferenceArray) -> Reference {
		let value = Rc::new_in(RefCell::new(array), &REFERENCE_ARRAY_ALLOCATOR);
		Reference::from_value(ReferencePtr::Array(ArrayPtr::Reference(value)))
	}

	pub fn allocate_interface(object: Object) -> Reference {
		Heap::it().allocate_interface_impl(object)
	}

	fn allocate_interface_impl(&self, object: Object) -> Reference {
		let value = Rc::new_in(RefCell::new(object), &INTERFACE_ALLOCATOR);
		Reference::from_value(ReferencePtr::Interface(value))
	}
}
