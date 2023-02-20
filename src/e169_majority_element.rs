pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    nums[nums.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = majority_element(vec![3, 2, 3]);
        assert_eq!(3, output);

        let output = majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
        assert_eq!(2, output);

        let output = majority_element(vec![0]);
        assert_eq!(0, output);
    }
}
