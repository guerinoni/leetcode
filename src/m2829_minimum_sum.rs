pub fn minimum_sum(n: i32, k: i32) -> i32 {
    let mut set = std::collections::HashSet::new();
    let mut counter = 1;
    let mut n = n;
    while n > 0 && counter <= k {
        if !set.contains(&(k - counter)) {
            set.insert(counter);
            n -= 1;
        }

        counter += 1;
    }

    set.iter().sum::<i32>() + (counter..counter + n).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(minimum_sum(5, 4), 18);
        assert_eq!(minimum_sum(2, 6), 3);
    }
}
