use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut mapping_s = HashMap::new();
    let mut mapping_t = HashMap::new();
    let iter = s.chars().zip(t.chars().into_iter());
    for (k, v) in iter {
        match (mapping_s.get(&k), mapping_t.get(&v)) {
            (Some(val), Some(key)) => {
                if v != *val || k != *key {
                    return false;
                }
            }
            (_, Some(_)) | (Some(_), _) => {
                return false;
            }
            _ => {
                let _ = mapping_s.insert(k, v);
                let _ = mapping_t.insert(v, k);
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(is_isomorphic("egg".to_owned(), "add".to_owned()));
        assert!(!is_isomorphic("foo".to_owned(), "bar".to_owned()));
        assert!(is_isomorphic("paper".to_owned(), "title".to_owned()));
        assert!(!is_isomorphic("badc".to_owned(), "baba".to_owned()));
    }
}
