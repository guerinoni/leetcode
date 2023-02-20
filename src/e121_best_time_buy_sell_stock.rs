pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((0, std::i32::MAX), |(profit, min), &n| {
            let m = std::cmp::min(min, n);
            (std::cmp::max(profit, n - min), m)
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(5, output);

        let output = max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(0, output);

        let output = max_profit(vec![2, 4, 1]);
        assert_eq!(2, output);
    }
}
