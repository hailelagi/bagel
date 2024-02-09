use std::alloc::Layout;
use std::ptr;

pub struct BumpAllocator {
    memory: *mut u8,
    next: usize,
    end: usize,
}

impl BumpAllocator {
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, std::mem::align_of::<usize>()).unwrap();
        let memory = unsafe { std::alloc::alloc(layout) as *mut u8 };
        BumpAllocator {
            memory,
            next: memory as usize,
            end: (memory as usize).wrapping_add(size),
        }
    }

    pub fn allocate(&mut self, size: usize, align: usize) -> *mut u8 {
        let layout = Layout::from_size_align(size, align).unwrap();
        let current = self.next;
        let new_ptr = ((current + align - 1) & !(align - 1)).max(self.next);
        if new_ptr + size <= self.end {
            self.next = new_ptr + size;
            new_ptr as *mut u8
        } else {
            ptr::null_mut()
        }
    }

    pub fn deallocate(&mut self, _ptr: *mut u8, _size: usize, _align: usize) {
        panic!("OOM")
    }
}
