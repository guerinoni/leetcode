pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }

    let fmt_values = |arg_1: i32, arg_2: i32| {
        if arg_1 == arg_2 {
            format!("{}", arg_1)
        } else {
            format!("{}->{}", arg_1, arg_2)
        }
    };
    let mut v = vec![nums[0]];
    let mut vv = Vec::with_capacity(2);
    for two in nums.windows(2) {
        if two[0] + 1 != two[1] {
            v.push(two[0]);
            vv.push(fmt_values(v[0], v[1]));
            v.clear();
            v.push(two[1]);
        }
    }

    v.push(nums[nums.len() - 1]);
    vv.push(fmt_values(v[0], v[1]));

    vv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = summary_ranges(vec![0, 1, 2, 4, 5, 7]);
        let expected = vec!["0->2", "4->5", "7"];
        assert_eq!(output, expected);

        let output = summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]);
        let expected = vec!["0", "2->4", "6", "8->9"];
        assert_eq!(output, expected);
    }
}
