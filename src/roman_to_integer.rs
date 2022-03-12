fn decode(symbol: char) -> i32 {
    match symbol {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!(""),
    }
}

pub fn roman_to_int(s: String) -> i32 {
    s.as_bytes()
        .windows(2)
        .map(|couple| {
            let left = decode(couple[0] as char);
            let right = decode(couple[1] as char);
            if right > left {
                -left
            } else {
                left
            }
        })
        .sum::<i32>()
        + decode(*s.as_bytes().last().unwrap() as char)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
