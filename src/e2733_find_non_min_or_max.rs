// Given an integer array nums containing distinct positive integers,
// find and return any number from the array that is neither the minimum nor the maximum value in the array,
// or -1 if there is no such number.

pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return -1;
    }

    let (mut min, mut max) = {
        if nums[0] < nums[1] {
            (nums[0], nums[1])
        } else {
            (nums[1], nums[0])
        }
    };

    let mut any_valid = nums[2];

    for n in nums.iter().skip(2) {
        if *n < min {
            any_valid = min;
            min = *n;
        }

        if *n > max {
            any_valid = max;
            max = *n;
        }
    }

    any_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(find_non_min_or_max(vec![3, 2, 1, 4]), 3);
        assert_eq!(find_non_min_or_max(vec![1, 2]), -1);
        assert_eq!(find_non_min_or_max(vec![2, 1, 3]), 2);
        assert_eq!(find_non_min_or_max(vec![3, 30, 24]), 24);
    }
}
