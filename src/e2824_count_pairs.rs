// NOTE: Brute force solution.
// pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
//     let mut pairs = 0;
//     for i in 0..nums.len() {
//         for j in i + 1..nums.len() {
//             if (nums[i] + nums[j]) < target && i < j {
//                 pairs += 1;
//             }
//         }
//     }

//     pairs
// }

pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut pairs = 0_i32;

    let mut numbers = nums;
    numbers.sort_unstable();

    let mut low = 0;
    let mut high = numbers.len() - 1;
    while low < high {
        if numbers[low] + numbers[high] < target {
            pairs += (high - low) as i32;
            low += 1;
        } else {
            high -= 1;
        }
    }

    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
        assert_eq!(count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
    }
}
