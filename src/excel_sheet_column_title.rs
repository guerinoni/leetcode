pub fn convert_to_title(column_number: i32) -> String {
    let mut ret = String::from("");
    let mut val = column_number;
    while val > 0 {
        val -= 1;
        let reminder = (val % 26) as u8;
        ret.insert(0, (b'A' + reminder) as char);
        val /= 26;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = convert_to_title(1);
        assert_eq!("A".to_owned(), output);

        let output = convert_to_title(28);
        assert_eq!("AB".to_owned(), output);

        let output = convert_to_title(701);
        assert_eq!("ZY".to_owned(), output);

        let output = convert_to_title(2147483647);
        assert_eq!("FXSHRXW".to_owned(), output);
    }
}
