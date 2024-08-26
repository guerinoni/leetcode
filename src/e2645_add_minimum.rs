// Given a string word to which you can insert letters "a", "b" or "c" anywhere and any number of times,
// return the minimum number of letters that must be inserted so that word becomes valid.
//
// A string is called valid if it can be formed by concatenating the string "abc" several times.

pub fn add_minimum(word: String) -> i32 {
    let mut inserted = 0;

    let mut w = word.chars().collect::<Vec<char>>();

    let mut expected = 'a';
    let mut index = 0_usize;
    loop {
        if index >= w.len() {
            let latest = w.last().unwrap();
            if latest == &'c' {
                // gracefully exit
                break;
            }

            match expected {
                'a' => inserted += 3,
                'b' => inserted += 2,
                'c' => inserted += 1,
                _ => {}
            }
            break;
        }

        let current = w[index];

        match (current, expected) {
            ('a', 'a') => expected = 'b',
            ('a', 'b') => {
                inserted += 1;
                w.insert(index, 'b');
                continue;
            }
            ('a', 'c') => {
                inserted += 1;
                w.insert(index, 'c');
                continue;
            }
            ('b', 'b') => expected = 'c',
            ('b', 'a') => {
                inserted += 1;
                w.insert(index, 'a');
                continue;
            }
            ('b', 'c') => {
                inserted += 1;
                w.insert(index, 'c');
                continue;
            }
            ('c', 'c') => expected = 'a',
            ('c', 'a') => {
                inserted += 1;
                w.insert(index, 'a');
                continue;
            }
            ('c', 'b') => {
                inserted += 1;
                w.insert(index, 'b');
                continue;
            }
            _ => {}
        }

        index += 1;
    }

    inserted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(add_minimum("b".to_string()), 2);
        assert_eq!(add_minimum("aaa".to_string()), 6);
        assert_eq!(add_minimum("abc".to_string()), 0);
    }
}
