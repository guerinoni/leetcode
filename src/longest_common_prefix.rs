pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

    let not_empty = strs.iter().filter(|&v| !v.is_empty()).count();
    if not_empty != strs.len() {
        return "".to_string();
    }

    let mut idx = 1;
    let first = strs.get(0).unwrap().as_str();
    loop {
        if idx > first.len() {
            break;
        }

        for s in strs.iter() {
            if !s.starts_with(&first[0..idx]) {
                return first[0..idx - 1].to_string();
            }
        }

        idx += 1;
    }

    first[0..idx - 1].to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(longest_common_prefix(vec![]), "".to_string());
        assert_eq!(
            longest_common_prefix(vec!["ab".to_string(), "a".to_string()]),
            "a".to_string()
        );

        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );

        assert_eq!(
            longest_common_prefix(vec![
                "asd".to_string(),
                "asd".to_string(),
                "asd".to_string()
            ]),
            "asd".to_string()
        );

        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}
