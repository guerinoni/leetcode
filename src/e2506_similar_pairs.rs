pub fn similar_pairs(words: Vec<String>) -> i32 {
    let mut h = std::collections::HashMap::new();
    for w in words {
        let mut n = vec![0; 26];
        for b in w.bytes() {
            let a = b - b'a';
            n[a as usize] = 1;
        }

        let e = h.entry(n).or_insert(0);
        *e += 1;
    }

    h.values().map(|x| x * (x - 1) / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(
            similar_pairs(vec![
                "dcedceadceceaeddcedc".to_string(),
                "dddcebcedcdbaeaaaeab".to_string(),
                "eecbeddbddeadcbbbdbb".to_string(),
                "decbcbebbddceacdeadd".to_string(),
                "ccbddbaedcadedbcaaae".to_string(),
                "dddcaadaceaedcdceedd".to_string(),
                "bbeddbcbbccddcaceeea".to_string(),
                "bdabacbbdadabbbddaea".to_string()
            ]),
            16
        );

        assert_eq!(
            similar_pairs(vec!["aabb".to_string(), "ab".to_string(), "ba".to_string(),]),
            3
        );

        assert_eq!(
            similar_pairs(vec![
                "aba".to_string(),
                "aabb".to_string(),
                "abcd".to_string(),
                "bac".to_string(),
                "aabc".to_string()
            ]),
            2
        );

        assert_eq!(
            similar_pairs(vec![
                "nba".to_string(),
                "cba".to_string(),
                "dba".to_string()
            ]),
            0
        );
    }
}
