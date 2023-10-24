use std::{rc::Rc, cell::RefCell};

use crate::jvm::types::{Value, reference::Reference, ReferenceableTypes};

use self::linked_list_allocator::LinkedListAllocator;

use super::locked::Locked;

pub(self) mod linked_list_allocator;

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

pub type ClassPtr = Rc<RefCell<ReferenceableTypes>, &'static Locked<LinkedListAllocator>>;
pub type ArrayPtr = Rc<RefCell<ReferenceableTypes>, &'static Locked<LinkedListAllocator>>;
pub type InterfacePtr = Rc<RefCell<ReferenceableTypes>, &'static Locked<LinkedListAllocator>>;

static CLASS_ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
static ARRAY_ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());
static INTERFACE_ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

static mut INSTANCE: Option<Heap> = None;

const HEAP_START: usize = 0x10000000; // 256 MB
const HEAP_SIZE: usize = 1024 * 1024; // 1 MB

pub struct Heap {
	class_allocator: &'static Locked<LinkedListAllocator>,
	array_allocator: &'static Locked<LinkedListAllocator>,
	interface_allocator: &'static Locked<LinkedListAllocator>,
}

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
		let class_allocator = &CLASS_ALLOCATOR;
		let array_allocator = &ARRAY_ALLOCATOR;
		let interface_allocator = &INTERFACE_ALLOCATOR;

		unsafe {
			class_allocator.lock().init(HEAP_START, HEAP_SIZE);
			array_allocator.lock().init(HEAP_START + HEAP_SIZE, HEAP_SIZE);
			interface_allocator.lock().init(HEAP_START + 2 * HEAP_SIZE, HEAP_SIZE);
		}

		Heap {
			class_allocator,
			array_allocator,
			interface_allocator,
		}
	}

	fn allocate_impl(&self, value: ReferenceableTypes) -> Reference {
		match value {
			ReferenceableTypes::Class(_) => {
				let value = Rc::new_in(RefCell::new(value), self.class_allocator);
				Reference::from_value(ReferencePtr::Class(value))
			},
			ReferenceableTypes::Array(_) => {
				let value = Rc::new_in(RefCell::new(value), self.array_allocator);
				Reference::from_value(ReferencePtr::Array(value))
			},
			ReferenceableTypes::Interface(_) => {
				let value = Rc::new_in(RefCell::new(value), self.interface_allocator);
				Reference::from_value(ReferencePtr::Interface(value))
			},
		}
	}
}
