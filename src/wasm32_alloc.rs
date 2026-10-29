//! The allocator `src/wasm32_new_delete.cpp` forwards C++'s operator new and delete to.

use std::alloc::{Layout, alloc, dealloc};

/// Each block starts with its size, which `dealloc` needs; 16 bytes keeps the rest aligned.
const HEADER: usize = 16;

#[unsafe(no_mangle)]
unsafe extern "C" fn meshopt_rs_alloc(size: usize) -> *mut u8 {
    let Some(total) = size.checked_add(HEADER) else {
        return std::ptr::null_mut();
    };
    let Ok(layout) = Layout::from_size_align(total, HEADER) else {
        return std::ptr::null_mut();
    };
    // SAFETY: the layout is never zero-sized, and the header fits in it.
    unsafe {
        let base = alloc(layout);
        if base.is_null() {
            return base;
        }
        base.cast::<usize>().write(total);
        base.add(HEADER)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn meshopt_rs_free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    // SAFETY: `ptr` came from `meshopt_rs_alloc`, which put the block's size just before it.
    unsafe {
        let base = ptr.sub(HEADER);
        let total = base.cast::<usize>().read();
        dealloc(base, Layout::from_size_align_unchecked(total, HEADER));
    }
}
