// NOTE: faster.
pub fn climb_stairs(n: i32) -> i32 {
    if n < 3 {
        return n;
    }

    let mut old_ways = 1;
    let mut ways = 2;
    for _ in 3..n + 1 {
        let temp = old_ways + ways;
        old_ways = ways;
        ways = temp;
    }

    ways
}

// NOTE: functional style.
// pub fn climb_stairs(n: i32) -> i32 {
//     (2..=n)
//         .into_iter()
//         .fold((1, 1), |(last1, last2), _| (last2, last1 + last2))
//         .1
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(climb_stairs(1), 1);
        assert_eq!(climb_stairs(2), 2);
        assert_eq!(climb_stairs(3), 3);
        assert_eq!(climb_stairs(4), 5);
        assert_eq!(climb_stairs(5), 8);
    }
}
