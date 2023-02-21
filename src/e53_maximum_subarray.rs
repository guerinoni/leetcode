// NOTE: linear, but functional style
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    nums.iter()
        .scan(i32::MIN, |curr: &mut i32, n: &i32| -> Option<i32> {
            *curr = if curr.is_negative() { *n } else { *curr + n };
            Some(*curr)
        })
        .max()
        .unwrap_or(0)
}

// NOTE: first try, O(n)
// pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//     let mut current_sum = 0;
// let mut sum = std::i32::MIN;
//
// for n in nums {
// if n <= current_sum + n {
// current_sum += n;
// } else {
// current_sum = n;
// }
//
// if current_sum > sum {
// sum = current_sum;
// }
// }
//
// sum
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(max_sub_array(vec![]), 0);
        assert_eq!(max_sub_array(vec![1]), 1);
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array(vec![0, 0, 0, 0, 0, 0, 0, 0]), 0);
    }
}
