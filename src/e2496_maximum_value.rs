pub fn maximum_value(strs: Vec<String>) -> i32 {
    strs.iter().map(|s| s.parse::<u32>().unwrap_or(s.len() as u32)).max().unwrap() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(
            maximum_value(vec![
                "alic3".to_string(),
                "bob".to_string(),
                "3".to_string(),
                "4".to_string(),
                "00000".to_string()
            ]),
            5
        );
        assert_eq!(
            maximum_value(vec![
                "1".to_string(),
                "01".to_string(),
                "001".to_string(),
                "0001".to_string()
            ]),
            1
        );
    }
}
