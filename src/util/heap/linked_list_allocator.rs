use std::{ptr::NonNull, mem, alloc::{Allocator, Layout, AllocError}};

use crate::util::{heap::linked_list_allocator::mem_util::align_up, locked::Locked};

use self::list_node::ListNode;

pub(self) mod list_node;
pub(self) mod mem_util;

pub type Ptr = NonNull<u8>;
pub type MemorySlice = NonNull<[u8]>;

pub struct LinkedListAllocator {
	head: ListNode,
}

impl LinkedListAllocator {
	pub const fn new() -> Self {
		Self {
			head: ListNode::new(0),
		}
	}

	pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
		self.add_free_region(heap_start, heap_size);
	}

	pub unsafe fn add_free_region(&mut self, address: usize, size: usize) {
		assert_eq!(align_up(address, mem::align_of::<ListNode>()), address);
        assert!(size >= mem::size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = address as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr)
	}

	pub fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
		let mut current = &mut self.head;
		while let Some(ref mut region) = current.next {
			if let Ok(start) = Self::alloc_from_region(&region, size, align) {
				let next = region.next.take();
				let ret = Some((current.next.take().unwrap(), start));
				current.next = next;
				return ret;
			} else {
				current = current.next.as_mut().unwrap();
			}
		}
		None
	}

	pub fn alloc_from_region(region: &ListNode, size: usize, align: usize) -> Result<usize, ()> {
		let start = align_up(region.start(), align);
		let end = start.checked_add(size).ok_or(())?;
		if end > region.end() {
			return Err(());
		}
		let excess_size = region.end() - end;
		if excess_size > 0 && excess_size < mem::size_of::<ListNode>() {
			return Err(());
		}
		Ok(start)
	}

	pub fn size_align(layout: Layout) -> (usize, usize) {
		let layout = layout.align_to(mem::align_of::<ListNode>()).expect("failed to adjust alignment").pad_to_align();
		let size = layout.size().max(mem::size_of::<ListNode>());
		(size, layout.align())
	}
}

unsafe impl Allocator for Locked<LinkedListAllocator> {
	fn allocate(&self, layout: Layout) -> Result<MemorySlice, AllocError> {
		let (size, align) = LinkedListAllocator::size_align(layout);
		let mut allocator = self.lock();

		if let Some((region, start)) = allocator.find_region(size, align) {
			let end = start.checked_add(size).expect("ptr overflow while alloc");
			let excess_size = region.end() - end;
			if excess_size > 0 {
				unsafe {
					allocator.add_free_region(end, excess_size);
				}
			}
			let start = match NonNull::new(start as *mut u8) {
				Some(v) => v,
				None => return Err(AllocError),
			};
			return Ok(NonNull::slice_from_raw_parts(start, size));
		}

		Err(AllocError)
	}

	unsafe fn deallocate(&self, ptr: Ptr, layout: Layout) {
		let (size, _) = LinkedListAllocator::size_align(layout);
		self.lock().add_free_region(ptr.addr().into(), size);
	}
}
