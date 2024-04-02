pub struct Arena<T> {
    data: Vec<T>,
    next_index: usize,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Arena {
            data: Vec::new(),
            next_index: 0,
        }
    }

    pub fn allocate(&mut self, value: T) -> usize {
        let index = self.next_index;
        self.data.push(value);
        self.next_index += 1;
        index
    }

    pub fn deallocate(&mut self, index: usize) {
        todo!()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }
}