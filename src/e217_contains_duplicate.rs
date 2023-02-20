// NOTE: Without any extra data structure
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut n = nums;
    n.sort_unstable();
    n.windows(2).any(|both| both[0] == both[1])
}

// use std::collections::HashSet;

// pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//     let mut set = HashSet::new();
//     for i in nums {
//         match set.get(&i) {
//             Some(_) => return true,
//             None => set.insert(i),
//         };
//     }

//     false
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(contains_duplicate(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 9]));
        assert!(contains_duplicate(vec![
            10, 11, 9, 8, 7, 6, 5, 4, 3, 2, 2, 1
        ]));
    }
}
