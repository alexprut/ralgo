use crate::datastructures::linked_list::LinkedList;

struct Stack<'a, T> {
    list: &'a LinkedList<'a, T>
}

impl<T> Stack<'_, T> {
    pub fn peek(&self) -> T {
        let tmp = pop();
        push(tmp);
        return tmp;
    }

    pub fn push(&self, value: T) {
        self.list.insertFront(value);
    }

    pub fn size(&self) -> usize {
        return self.list.size();
    }

    pub fn empty(&self) -> bool {
        return self.list.empty();
    }

    pub fn pop(&mut self) -> T {
        return self.list.removeFront();
    }

    pub fn insert(&mut self, value: T) {
        self.push(value);
    }

    pub fn remove(&mut self) {
        self.pop();
    }
}
