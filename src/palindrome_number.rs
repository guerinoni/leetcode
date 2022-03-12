// NOTE: easier to read, but slower and allocates 2 Strings.
// pub fn is_palindrome(x: i32) -> bool {
//     let s = x.to_string();
//     let reverse: String = s.chars().rev().collect();
//     reverse.eq(&s)
// }

// NOTE: only with math and no string alloc.
// pub fn is_palindrome(x: i32) -> bool {
//     if x.is_negative() {
//         return false;
//     }

//     let mut n = x;
//     let mut rev = 0;
//     loop {
//         let last_digit = n % 10;
//         rev = rev * 10 + last_digit;
//         n = n / 10;
//         if n <= 0 {
//             break;
//         }
//     }

//     x == rev
// }

// NOTE: first try.
// pub fn is_palindrome(x: i32) -> bool {
//     if x.is_negative() {
//         // avoid allocation on negative :)
//         return false;
//     }

//     let s = x.to_string();
//     for i in 0..s.len() {
//         let f = s.bytes().nth(i);
//         let b = s.bytes().nth_back(i);
//         match (f, b) {
//             (Some(f), Some(b)) => {
//                 if f != b {
//                     return false;
//                 }
//             }
//             _ => return false,
//         }
//     }

//     true
// }

// NOTE: improve speed but allocate vector instead of string.
pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    if x < 10 {
        return true;
    }

    let mut digits = vec![];
    let mut num: i32 = x;

    while num != 0 {
        digits.push(num % 10);
        num /= 10;
    }

    let mut i = 0;
    let mut j = digits.len() - 1;

    while i < j && digits[i] == digits[j] {
        i += 1;
        j -= 1;
    }

    i >= j
}

#[cfg(test)]
mod test {
    use super::is_palindrome;

    #[test]
    fn check() {
        assert!(is_palindrome(121));
        assert!(is_palindrome(9));
        assert!(!is_palindrome(12));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(12345678));
    }
}
