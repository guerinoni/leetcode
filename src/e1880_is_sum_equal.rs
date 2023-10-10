pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
    let fw = first_word
        .chars()
        .fold(0, |acc, c| acc * 10 + (c as u8 - 'a' as u8));

    let sw = second_word
        .chars()
        .fold(0, |acc, c| acc * 10 + (c as u8 - 'a' as u8));

    let tw = target_word
        .chars()
        .fold(0, |acc, c| acc * 10 + (c as u8 - 'a' as u8));

    fw + sw == tw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sum_equal() {
        assert_eq!(
            is_sum_equal("acb".to_string(), "cba".to_string(), "cdb".to_string()),
            true
        );
        assert_eq!(
            is_sum_equal("aaa".to_string(), "a".to_string(), "aab".to_string()),
            false
        );
        assert_eq!(
            is_sum_equal("aaa".to_string(), "a".to_string(), "aaaa".to_string()),
            true
        );
    }
}
