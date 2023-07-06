pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut n = vec![0; nums.len() + 1];
    for i in nums {
        n[i as usize] = 1;
    }

    n.iter().position(|&x| x == 0).unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
        assert_eq!(missing_number(vec![0, 1]), 2);
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
