// NOTE: easy solution with rust api
// pub fn str_str(haystack: String, needle: String) -> i32 {
//     match haystack.find(needle.as_str()) {
//         Some(index) => index.try_into().unwrap(),
//         None => -1,
//     }
// }

// NOTE: functional way
pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack
        .find(&needle)
        .map(|n| n.try_into().unwrap())
        .unwrap_or(-1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(str_str("".to_string(), "".to_string()), 0);
        assert_eq!(str_str("aaaaa".to_string(), "bba".to_string()), -1);
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    }
}
