pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    (left..=right).filter(|&n| is_self_diving(n)).collect()
}

fn is_self_diving(n: i32) -> bool {
    let mut nn = n;
    while nn > 0 {
        let digit = nn % 10;
        if digit == 0 || n % digit != 0 {
            return false;
        }

        nn /= 10;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(
            self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
        assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
    }
}
