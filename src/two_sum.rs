use std::collections::HashMap;

// NOTE: O^2 but space complexity O(1)
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for i in 0..nums.len() - 1 {
//         for j in 1..nums.len() {
//             if nums[i] + nums[j] == target {
//                 return vec![i as i32, j as i32];
//             }
//         }
//     }

//     Vec::new()
// }

// NOTE: O(n) but space complexity O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, value) in nums.iter().enumerate() {
        let value_already_found = target - value;
        if let Some(v) = map.get(&value_already_found) {
            return vec![*v as i32, i as i32];
        }

        map.insert(value, i);
    }

    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 3], 6), vec![0, 2]);
        assert_eq!(
            two_sum(
                vec![28, 1, 24, 2, 7, 18, 10, 27, 11, 9, 8, 3, 12, 5, 29, 30],
                59
            ),
            vec![14, 15]
        );
    }
}
