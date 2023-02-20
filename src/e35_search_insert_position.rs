// NOTE: eheheh
// pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
//     match nums.binary_search(&target) {
//         Ok(idx) => idx.try_into().unwrap(),
//         Err(idx) => idx.try_into().unwrap(),
//     }
// }

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // since are sorted...
    if target < nums[0] {
        return 0;
    }

    let mut left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let middle = left + (right - left) / 2;

        if nums[middle] == target {
            return middle.try_into().unwrap();
        }

        if nums[middle] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    left.try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3], 4), 2);
        assert_eq!(search_insert(vec![1, 3, 5], 4), 2);
        assert_eq!(search_insert(vec![2, 7, 8, 9, 10], 9), 3);
        assert_eq!(search_insert(vec![3, 5, 7, 9, 10], 8), 3);
    }
}
