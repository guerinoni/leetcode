pub fn count_binary_substrings(s: String) -> i32 {
    let len = s.len();
    let b = s.into_bytes();
    let mut i = 0;
    let mut counter = 0;
    let mut prev_windows_size = 0;

    while i < len {
        let next_different = ((i + 1)..len).find(|&idx| b[i] != b[idx]).unwrap_or(len);
        let windows_size = next_different - i;
        counter += prev_windows_size.min(windows_size);
        prev_windows_size = windows_size;
        i = next_different;
    }

    counter as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e696_count_binary_substrings() {
        assert_eq!(count_binary_substrings("00110011".to_string()), 6);
        assert_eq!(count_binary_substrings("10101".to_string()), 4);
        assert_eq!(count_binary_substrings("00110".to_string()), 3);
    }
}
