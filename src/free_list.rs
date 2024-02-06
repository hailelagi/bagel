use std::alloc::{alloc, dealloc, Layout};

//
const INITIAL_BLOCKS: usize = 10;

// default size on 32 bit linux
const DEFAULT_BLOCK_SIZE: usize = 64;

struct ListNode {
    size: usize,
    next: Option<Box<ListNode>>,
}

pub struct FreeList {
    head: Option<Box<ListNode>>,
}

impl FreeList {
    pub fn new() -> Self {
        // Pre-allocate some memory blocks in the free list
        let mut head = None;
        for _ in 0..INITIAL_BLOCKS {
            let node = ListNode {
                next: head.take(),
                size: DEFAULT_BLOCK_SIZE,
            };
            head = Some(Box::new(node));
        }
        FreeList { head }
    }
    pub fn allocate(&mut self, size: usize, align: usize) -> *mut u8 {
        let aligned_size = size.next_power_of_two();
        let align_mask = align - 1;
        let layout_size = aligned_size + align_mask;

        match self.head.take() {
            Some(mut node) => {
                let node_ptr = node.as_mut() as *mut ListNode as usize;
                let aligned_ptr = (node_ptr + align_mask) & !align_mask;
                let offset = aligned_ptr - node_ptr;
                if offset <= node.size - layout_size {
                    unsafe { Some(Box::from_raw((aligned_ptr) as *mut ListNode)) }

                    return aligned_ptr as *mut u8;
                } else {
                    panic!("error during alignment")
                }
            }
            None => panic!("oh no! out of memory"),
        }
    }

    pub fn deallocate(&mut self, ptr: *mut u8, size: usize, align: usize) {
        let layout = Layout::from_size_align(size, align).unwrap();
        let new_node = Box::new(ListNode {
            size,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        unsafe { dealloc(ptr, layout) };
    }
}
