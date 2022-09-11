use std::collections::HashSet;

fn square_sum(n: i32) -> i32 {
    let mut n = n;
    let mut new_num = 0_i32;
    while n > 0 {
        let t = n % 10;
        new_num += t * t;
        n /= 10;
    }

    new_num
}

pub fn is_happy(n: i32) -> bool {
    let mut results = HashSet::new();
    let mut n = n;
    loop {
        let s = square_sum(n);
        if s == 1 {
            return true;
        }

        if results.contains(&s) {
            break;
        } else {
            results.insert(s);
        }

        n = s;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(is_happy(19));
        assert!(!is_happy(2));
        assert!(!is_happy(11));
    }
}
