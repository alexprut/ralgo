struct LinkedList<'a, T> {
    head: &'a Node<'a, T>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn insert_front(&mut self, value: T) {
        let next = self.head;
        self.head = &Node { value, next };
        size += 1;
    }

    pub fn empty(&self) -> bool {
        return self.size == 0;
    }

    pub fn remove_front(&mut self) -> T {
        size -= 1;
        let node = self.head;
        self.head = node.next;
        return *node.value;
    }

    pub fn head(&self) -> &Node<T> {
        return self.head;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }
}

struct Node<'a, T> {
    value: T,
    next: &'a Node<'a, T>,
}
