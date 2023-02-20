struct MyQueue {
    pub inner: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue { inner: vec![] }
    }

    fn push(&mut self, x: i32) {
        self.inner.push(x)
    }

    fn pop(&mut self) -> i32 {
        self.inner.remove(0)
    }

    fn peek(&mut self) -> i32 {
        *self.inner.first().unwrap()
    }

    fn empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut obj = MyQueue::new();
        obj.push(5);
        assert_eq!(obj.pop(), 5);
        assert!(obj.empty());
        obj.push(5);
        obj.push(6);
        assert_eq!(obj.peek(), 5);
        assert!(!obj.empty());

        let mut my_queue = MyQueue::new();
        my_queue.push(1);
        my_queue.push(2);
        assert_eq!(my_queue.peek(), 1);
        assert_eq!(my_queue.pop(), 1);
        assert!(!my_queue.empty());
    }
}
