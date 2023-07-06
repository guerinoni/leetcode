pub fn word_pattern(pattern: String, s: String) -> bool {
    let items = s.split_whitespace().collect::<Vec<&str>>();

    if items.len() != pattern.len() {
        return false;
    }

    let mut map_by_pattern = std::collections::HashMap::new();
    let mut map_by_items = std::collections::HashMap::new();

    pattern.chars().zip(items).all(|(ch, i)| {
        if map_by_pattern.entry(ch).or_insert(i) != &i {
            return false;
        }
        if map_by_items.entry(i).or_insert(ch) != &ch {
            return false;
        }

        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_e290_word_pattern() {
        assert_eq!(
            word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
            true
        );
        assert_eq!(
            word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
        assert_eq!(
            word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
            false
        );
        assert_eq!(
            word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
            false
        );
        assert_eq!(
            word_pattern("abab".to_string(), "dog cat cat dog".to_string()),
            false
        );
    }
}
