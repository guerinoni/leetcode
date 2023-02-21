pub fn is_ugly(n: i32) -> bool {
    if n == 0 {
        return false;
    }

    let mut n = n;
    [5, 3, 2].map(|v| {
        while n % v == 0 {
            n /= v;
        }
    });

    n == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(is_ugly(6));
        assert!(is_ugly(1));
        assert!(!is_ugly(14));
    }
}
