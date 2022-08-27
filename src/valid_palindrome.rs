use std::ops::ControlFlow;

pub fn is_palindrome(s: String) -> bool {
    let result = s
        .to_lowercase()
        .chars()
        .filter(|&b| b.is_ascii_alphanumeric())
        .collect::<String>();

    match result
        .chars()
        .zip(result.chars().rev())
        .try_for_each(|couple| {
            if couple.0 != couple.1 {
                return ControlFlow::Break(false);
            } else {
                return ControlFlow::Continue(());
            }
        }) {
        ControlFlow::Continue(()) => true,
        ControlFlow::Break(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_owned()));
        assert!(!is_palindrome("race a car".to_owned()));
        assert!(is_palindrome(" ".to_owned()));
    }
}
