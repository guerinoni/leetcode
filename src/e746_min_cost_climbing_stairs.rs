pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut last = 0;
    let mut prev = 0;
    for i in (0..cost.len()).rev() {
        let min = last.min(prev);
        prev = last;
        last = cost[i] + min;
    }
    last.min(prev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_746() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
