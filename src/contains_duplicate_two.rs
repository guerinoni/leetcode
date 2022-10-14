use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map = HashMap::new();
    for (index, v) in nums.iter().enumerate() {
        match map.insert(v, index) {
            Some(idx) if (index - idx) as i32 <= k => return true,
            _ => continue,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
        assert!(!contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
