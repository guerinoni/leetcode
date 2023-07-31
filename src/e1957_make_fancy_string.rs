pub fn make_fancy_string(s: String) -> String {
    if s.len() < 3 {
        return s;
    }

    let mut ss = s[0..2].to_string();

    for b in s.as_bytes().windows(3) {
        if b[0] != b[1] || b[1] != b[2] {
            ss.push(b[2] as char);
        }
    }

    ss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(
            make_fancy_string("leeetcode".to_string()),
            "leetcode".to_string()
        );
        assert_eq!(
            make_fancy_string("aaabaaaa".to_string()),
            "aabaa".to_string()
        );
        assert_eq!(make_fancy_string("aab".to_string()), "aab".to_string());
    }
}
