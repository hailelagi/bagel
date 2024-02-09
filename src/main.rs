mod bump;
mod free_list;

use bump::BumpAllocator;
use std::alloc::Layout;

fn main() {
    let mut allocator = BumpAllocator::new(1024); // Initialize the bump allocator with 1024 bytes
    let layout = Layout::from_size_align(16, 4).unwrap();
    let ptr = allocator.allocate(layout.size(), layout.align());

    if !ptr.is_null() {
        println!("Allocated memory at {:?}", ptr);
    } else {
        println!("Allocation failed!");
    }

    // TODO
    // let mut free_list = free_list::FreeList::new();

    // let ptr1 = free_list.allocate(64, 4);
    // let ptr2 = free_list.allocate(64, 4);

    // println!("Allocated pointers: {:?}, {:?}", ptr1, ptr2);

    // free_list.deallocate(ptr1, 64, 4);
    // free_list.deallocate(ptr2, 64, 4);
}
