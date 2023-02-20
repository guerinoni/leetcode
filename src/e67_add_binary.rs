// NOTE: first try
pub fn add_binary(a: String, b: String) -> String {
    let mut v = 1_u128;
    let mut aa = 0_u128;
    for c in a.as_bytes().iter().rev() {
        aa += if *c == b'1' { v } else { 0 };
        v *= 2;
    }

    v = 1;
    let mut bb = 0_u128;
    for c in b.as_bytes().iter().rev() {
        bb += if *c == b'1' { v } else { 0 };
        v *= 2;
    }

    format!("{:b}", aa + bb)
}

// NOTE: with dedicated API.
// pub fn add_binary(a: String, b: String) -> String {
//     format!(
//         "{:b}",
//         u128::from_str_radix(&a, 2).unwrap() + u128::from_str_radix(&b, 2).unwrap()
//     )
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(
            add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
        assert_eq!(
            add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );

        let a = "10100000100100110110010000010101111011011001101110111111111101000000101111001110001111100001101".to_string();
        let b = "110101001011101110001111100110001010100001101011101010000011011011001011101111001100000011011110011".to_string();
        let expected = "110111101100010011000101110110100000011101000101011001000011011000001100011110011010010011000000000".to_string();
        assert_eq!(add_binary(a, b), expected);
    }
}
