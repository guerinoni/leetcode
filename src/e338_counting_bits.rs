// NOTE: First try, iterative solution.
// pub fn count_bits(n: i32) -> Vec<i32> {
//     let mut v = vec![0; n as usize + 1];
//     for i in 0..=n {
//         v[i as usize] = i.count_ones() as i32;
//     }

//     v
// }

// NOTE: Reuse old calculated result with math trick
//  count_bits(0) = 0
//  count_bits(1) = count_bits(1 - 1) + 1 =  count_bits(0) + 1 = 0 + 1 = 1
//  count_bits(2) = count_bits(2 / 2)     =  count_bits(1)             = 1
//  count_bits(3) = count_bits(3 - 1) + 1 =  count_bits(2) + 1 = 1 + 1 = 2
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut v = vec![0; n as usize + 1];
    for i in 1..(n as usize + 1) {
        let t = if i % 2 == 0 { v[i / 2] } else { v[i - 1] + 1 };

        v[i] = t;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_338() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
