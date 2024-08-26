// You are given a string s consisting only of uppercase English letters.
// You can apply some operations to this string where, in one operation, you can remove any occurrence of one of the substrings "AB" or "CD" from s.
// Return the minimum possible length of the resulting string that you can obtain.
//
// Note that the string concatenates after removing the substring and could produce new "AB" or "CD" substrings.

pub fn min_length(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }

    let mut i = 0_usize;
    let mut j = 1_usize;

    let mut s = s.chars().collect::<Vec<char>>();

    loop {
        if j >= s.len() {
            break;
        }

        if (s[i] == 'A' && s[j] == 'B') || (s[i] == 'C' && s[j] == 'D') {
            s.remove(i);
            s.remove(i);

            if i > 0 {
                i -= 1;
                j -= 1;
            }

            continue;
        } else {
            i += 1;
            j += 1;
        }
    }

    s.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(min_length("ABFCACDB".to_string()), 2);
        assert_eq!(min_length("ACBBD".to_string()), 5);
        assert_eq!(min_length("CABABD".to_string()), 0);
    }
}
