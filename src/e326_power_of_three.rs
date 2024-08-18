/// Given an integer n, return true if it is a power of three. Otherwise, return false.
/// An integer n is a power of three, if there exists an integer x such that n == 3x.

pub fn is_power_of_three(n: i32) -> bool {
    if n == 1 {
        return true;
    }
    let mut nn = n as f64;
    loop {
        nn /= 3.0;

        if nn == 1.0 {
            return true;
        }

        if nn < 1.0 {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(is_power_of_three(27), true);
        assert_eq!(is_power_of_three(0), false);
        assert_eq!(is_power_of_three(9), true);
        assert_eq!(is_power_of_three(45), false);
        assert_eq!(is_power_of_three(1), true);
    }
}
