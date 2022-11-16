use std::collections::VecDeque;

struct MyStack(pub VecDeque<i32>);

impl MyStack {
    fn new() -> Self {
        Self(VecDeque::new())
    }

    fn push(&mut self, x: i32) {
        self.0.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        self.0.pop_back().unwrap_or(0)
    }

    fn top(&self) -> i32 {
        *self.0.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        let mut obj = MyStack::new();
        obj.push(1);
        assert_eq!(obj.top(), 1);
        assert_eq!(obj.pop(), 1);
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.top(), 2);
        assert!(!obj.empty());
    }
}
