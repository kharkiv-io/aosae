use fundsp::audiounit::AudioUnit;
use std::alloc::{self, Layout, alloc};
pub struct ResourceManager {
    alloc: Vec<*mut u8>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager { alloc: Vec::new() }
    }

    pub fn alloc_(&mut self, size: usize) -> *mut u8 {
        let align = size;
        let mem_block = std::alloc::Layout::from_size_align(size, align);
        let ptr = unsafe { std::alloc::alloc(mem_block.unwrap()) };
        self.alloc.push(ptr);
        ptr
    }

    pub fn dealloc_(&mut self, ptr: *mut u8, layout: Layout) {
        unsafe {
            alloc::dealloc(ptr, layout);
            self.alloc.retain(|p| *p != ptr);
        }
    }

    pub fn get_layout_alldeall_<T>() -> Layout {
        unsafe { Layout::from_size_align_unchecked(size_of::<T>(), size_of::<T>()) }
    }
}
