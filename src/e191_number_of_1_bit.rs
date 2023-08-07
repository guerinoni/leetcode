pub fn hamming_weight(n: u32) -> i32 {
    (0..32)
        .fold((0_i32, n), |(counter, number), _| {
            (counter + (number & 1 == 1) as i32, number >> 1)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = hamming_weight(13);
        assert_eq!(output, 3);

        let output = hamming_weight(15);
        assert_eq!(output, 4);

        let output = hamming_weight(255);
        assert_eq!(output, 8);
    }
}
