pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    words
        .iter()
        .filter(|w| {
            w.as_bytes()
                .iter()
                .take_while(|&&b| allowed.contains(b as char))
                .count()
                == w.len()
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = count_consistent_strings(
            "ab".into(),
            vec![
                "ad".into(),
                "bd".into(),
                "aaab".into(),
                "baa".into(),
                "badab".into(),
            ],
        );

        assert_eq!(output, 2);

        let output = count_consistent_strings(
            "abc".into(),
            vec![
                "a".into(),
                "b".into(),
                "c".into(),
                "ab".into(),
                "ac".into(),
                "bc".into(),
                "abc".into(),
            ],
        );
    }
}
