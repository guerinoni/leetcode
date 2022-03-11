// use std::collections::HashMap;

// NOTE: first try.
// pub fn is_valid(s: String) -> bool {
//     let mut v = vec![];
//     let m = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
//     for c in s.chars() {
//         match m.get(&c) {
//             Some(k) => match v.pop() {
//                 Some(p) if p == *k => continue,
//                 _ => return false,
//             },
//             None => v.push(c),
//         }
//     }

//     v.is_empty()
// }

// NOTE: this can be inlined (compiler explorer will be happy :) )
fn reverse(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        _ => None,
    }
}

pub fn is_valid(s: String) -> bool {
    let mut v = vec![];
    for c in s.chars() {
        match reverse(c) {
            Some(k) => match v.pop() {
                Some(p) if p == k => continue,
                _ => return false,
            },
            None => v.push(c),
        }
    }

    v.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("[)".to_string()), false);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("".to_string()), true);
        assert_eq!(is_valid("{[()]}".to_string()), true);
    }
}
