pub fn check_ones_segment(s: String) -> bool {
    s.as_bytes().windows(2).all(|w| w != b"01")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(check_ones_segment("1001".to_string()), false);
        assert_eq!(check_ones_segment("110".to_string()), true);
    }
}
