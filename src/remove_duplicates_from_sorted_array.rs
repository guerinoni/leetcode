// NOTE: first try
// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     let mut idx = 0;

//     loop {
//         if idx + 1 >= nums.len() {
//             break;
//         }

//         if nums[idx] == nums[idx + 1] {
//             nums.remove(idx);
//         } else {
//             idx += 1;
//         }
//     }

//     nums.len() as i32
// }

// NOTE: no shift of vector
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut last_idx_worth = 0;
    for idx in 1..nums.len() {
        if nums[last_idx_worth] < nums[idx] {
            last_idx_worth += 1;
            nums.swap(last_idx_worth, idx);
        }
    }

    (last_idx_worth + 1).try_into().unwrap()
}

#[cfg(test)]
mod test {
    use crate::remove_duplicates_from_sorted_array::remove_duplicates;

    #[test]
    fn check() {
        assert_eq!(remove_duplicates(&mut vec![]), 0);
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(
            remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );

        assert_eq!(
            remove_duplicates(&mut vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            1
        );

        assert_eq!(
            remove_duplicates(&mut vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
            10
        );
    }
}
