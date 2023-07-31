pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut num_to_sorted_idx = std::collections::HashMap::new();

    let mut sorted_nums = nums.clone();
    sorted_nums.sort_unstable();

    sorted_nums.iter().enumerate().for_each(|(idx, &n)| {
        num_to_sorted_idx.entry(n).or_insert(idx);
    });

    nums.iter()
        .map(|n| *num_to_sorted_idx.get(n).unwrap() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check() {
        assert_eq!(
            smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
        assert_eq!(
            smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }
}
