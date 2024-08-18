pub fn is_power_of_four(n: i32) -> bool {
    let mut nn = n as f64;
    loop {
        if nn == 1.0 {
            return true;
        }

        nn /= 4.0;

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
        assert_eq!(is_power_of_four(16), true);
        assert_eq!(is_power_of_four(5), false);
        assert_eq!(is_power_of_four(1), true);
        assert_eq!(is_power_of_four(0), false);
        assert_eq!(is_power_of_four(4), true);
    }
}
