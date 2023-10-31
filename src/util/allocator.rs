use std::{alloc::{Allocator, Layout, AllocError}, ptr::NonNull};

use libc::{malloc, free};

pub struct JVMAllocator {}

unsafe impl Allocator for JVMAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let ptr = unsafe { malloc(layout.size()) as *mut u8 };
		if ptr.is_null() {
			Err(AllocError)
		} else {
			Ok(NonNull::slice_from_raw_parts(NonNull::new(ptr).unwrap(), layout.size()))
		}
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _: Layout) {
        free(ptr.as_ptr() as *mut libc::c_void);
    }
}