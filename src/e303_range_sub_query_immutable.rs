struct NumArray {
    v: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { v: nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.v[left as usize..=right as usize].iter().sum()
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        let ret_1 = obj.sum_range(0, 2);
        let ret_2 = obj.sum_range(2, 5);
        let ret_3 = obj.sum_range(0, 5);

        assert_eq!(ret_1, 1);
        assert_eq!(ret_2, -1);
        assert_eq!(ret_3, -3);
    }
}
