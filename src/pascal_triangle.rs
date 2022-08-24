pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut counter = 1;
    let mut ret = vec![vec![1]];
    while counter < num_rows {
        let last = match ret.last() {
            Some(v) => v,
            None => break,
        };

        let mut to_push = vec![last[0]];
        to_push.extend(last.windows(2).map(|v| v[0] + v[1]));
        to_push.push(match last.last() {
            Some(v) => *v,
            None => break,
        });

        ret.push(to_push);
        counter += 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = generate(1);
        let expected = vec![vec![1]];
        assert_eq!(output, expected);

        let output = generate(2);
        let expected = vec![vec![1], vec![1, 1]];
        assert_eq!(output, expected);

        let output = generate(5);
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(output, expected);
    }
}
