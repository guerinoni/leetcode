use std::collections::HashMap;

// NOTE: With an hashmap.
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut h = HashMap::new();
    for i in s.chars() {
        h.entry(i).and_modify(|c| *c += 1).or_insert(1);
    }

    for i in t.chars() {
        if h.contains_key(&i) {
            h.entry(i).and_modify(|c| *c -= 1);
        } else {
            return false;
        }
    }

    for i in h.iter() {
        if i.1 > &0 {
            return false;
        }
    }

    true
}

// NOTE: Alloc 2 vec + sort and compare.
// From the benches has better performance. :)
// pub fn is_anagram(s: String, t: String) -> bool {
//     if s.len() != t.len() {
//         return false;
//     }

//     let mut v1 = s.chars().collect::<Vec<char>>();
//     let mut v2 = t.chars().collect::<Vec<char>>();

//     v1.sort_unstable();
//     v2.sort_unstable();

//     v1 == v2
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(is_anagram("anagram".to_owned(), "nagaram".to_owned()));
        assert!(!is_anagram("rat".to_owned(), "car".to_owned()));
    }
}
