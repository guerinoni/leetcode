pub fn modify_string(s: String) -> String {
    let mut ss = vec![];
    for (idx, v) in s.as_bytes().iter().enumerate() {
        if *v != b'?' {
            ss.push(*v);
            continue;
        }

        let l = if idx == 0 { None } else { Some(ss[idx - 1]) };
        let r = if idx == s.len() - 1 {
            None
        } else {
            if idx >= ss.len() {
                Some(s.as_bytes()[idx + 1])
            } else {
                Some(ss[idx + 1])
            }
        };

        let mut c = b'a';
        while Some(c) == r || Some(c) == l {
            c += 1;
        }

        ss.push(c)
    }

    String::from_utf8(ss).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(modify_string("?zs".to_string()), "azs".to_string());
        assert_eq!(modify_string("ubv?w".to_string()), "ubvaw".to_string());
        assert_eq!(modify_string("j?qg??b".to_string()), "jaqgacb".to_string());
    }
}
