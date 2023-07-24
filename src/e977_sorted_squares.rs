// NOTE: Super easy in first try, but I think it can be optimized.
// pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//     let mut v = nums.iter().map(|x| x * x).collect::<Vec<i32>>();
//     v.sort();
//     v
// }

pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut v = vec![0; nums.len()];
    let mut low = 0;
    let mut high = nums.len() - 1;

    for i in (0..nums.len()).rev() {
        let n = if nums[low].abs() > nums[high].abs() {
            let t = nums[low] * nums[low];
            low += 1;
            t
        } else {
            let t = nums[high] * nums[high];
            high -= 1;
            t
        };

        v[i] = n;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            sorted_squares(vec![-4, -1, 0, 3, 10]),
            vec![0, 1, 9, 16, 100]
        );
        assert_eq!(
            sorted_squares(vec![-7, -3, 2, 3, 11]),
            vec![4, 9, 9, 49, 121]
        );
    }
}
