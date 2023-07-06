pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut j = 0;
    while j <= nums.len() - 1 {
        if nums[j] != 0 {
            nums.swap(i, j);
            i += 1;
        }
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_e283_move_zeroes() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![0, 0, 1];
        move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 0, 0]);
    }
}
