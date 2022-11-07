// Canonical solution.
pub fn is_power_of_two(n: i32) -> bool {
    if n == 1 {
        return true;
    }

    let mut nn = n;
    while nn > 2 {
        if nn % 2 != 0 {
            return false;
        }
        nn /= 2;
    }

    nn == 2
}

// Math solution :)
// pub fn is_power_of_two(n: i32) -> bool {
// n > 0 && n & (n - 1) == 0
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(16));
        assert!(!is_power_of_two(-16));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(6));
    }
}
