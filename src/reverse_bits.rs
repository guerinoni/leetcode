// NOTE: this is rust API :)
// pub fn reverse_bits(x: u32) -> u32 {
//     x.reverse_bits()
// }

// NOTE: with string conversion, some allocation involved.
// pub fn reverse_bits(x: u32) -> u32 {
//     let x = format!("{:0>32b}", x).chars().rev().collect::<String>();
//     u32::from_str_radix(&x, 2).unwrap()
// }

// NOTE: numeric approach:
// - take number from input and every iteration shift to right of 1
// - in this way we can evaluate every iteration only the LSB
// - and push to "sum" variable as 0 or 1 based what we found
pub fn reverse_bits(x: u32) -> u32 {
    (0..32)
        .fold((0, x), |(sum, n), _| (sum << 1 | n & 1, n >> 1))
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = reverse_bits(43261596);
        assert_eq!(964176192, output);

        let output = reverse_bits(4294967293);
        assert_eq!(3221225471, output);
    }
}
