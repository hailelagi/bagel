mod free_list;

fn main() {
    let mut free_list = free_list::FreeList::new();

    let ptr1 = free_list.allocate(64, 4);
    let ptr2 = free_list.allocate(64, 4);

    println!("Allocated pointers: {:?}, {:?}", ptr1, ptr2);

    free_list.deallocate(ptr1, 64, 4);
    free_list.deallocate(ptr2, 64, 4);
}
