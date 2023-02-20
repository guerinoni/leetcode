pub fn title_to_number(column_title: String) -> i32 {
    column_title
        .into_bytes()
        .into_iter()
        .fold(0_i32, |sum, letter| sum * 26 + (letter - 65 + 1) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(1, title_to_number("A".to_owned()));
        assert_eq!(28, title_to_number("AB".to_owned()));
        assert_eq!(701, title_to_number("ZY".to_owned()));
    }
}
