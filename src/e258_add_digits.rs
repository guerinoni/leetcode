// NOTE: More alloc for constructing string.
pub fn add_digits(num: i32) -> i32 {
    let mut nums = format!("{num}");
    while nums.len() > 1 {
        let s: i32 = nums.bytes().map(|c| c as i32 - 48).sum();
        nums = format!("{s}");
    }

    nums.parse::<i32>().unwrap()
}

// pub fn add_digits(num: i32) -> i32 {
//     let mut n = num;
//     while n > 9 {
//         let mut sum = 0;
//         while n != 0 {
//             sum += n % 10;
//             n /= 10;
//         }
//         n = sum;
//     }
//
//     n
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(2, add_digits(38));
        assert_eq!(8, add_digits(8));
    }
}
