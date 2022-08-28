use std::ops::ControlFlow;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let v = nums.chunks(2).try_for_each(|both| {
        if both.len() == 1 || (both.len() == 2 && both[0] != both[1]) {
            ControlFlow::Break(both[0])
        } else {
            ControlFlow::Continue(())
        }
    });

    match v {
        ControlFlow::Break(v) => v,
        ControlFlow::Continue(()) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let output = single_number(vec![2, 2, 1]);
        assert_eq!(1, output);
        let output = single_number(vec![4, 1, 2, 1, 2]);
        assert_eq!(4, output);
        let output = single_number(vec![1]);
        assert_eq!(1, output);
    }
}
