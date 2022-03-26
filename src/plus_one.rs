use std::ops::ControlFlow;

// NOTE: performance seems better with iterator.
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut ret = digits;
    ret.iter_mut().rev().try_for_each(|v| {
        if *v + 1 == 10 {
            *v = 0;
            ControlFlow::Continue(())
        } else {
            *v += 1;
            ControlFlow::Break(())
        }
    });

    if *ret.first().unwrap() == 0 {
        ret.insert(0, 1);
    }

    ret
}

// NOTE: classic with for loop
// pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
//     let mut ret = digits;
//     for value in ret.iter_mut().rev() {
//         if *value + 1 == 10 {
//             *value = 0;
//         } else {
//             *value += 1;
//             break;
//         }
//     }

//     if *ret.first().unwrap() == 0 {
//         ret.insert(0, 1);
//     }

//     ret
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(plus_one(vec![1, 2, 9]), vec![1, 3, 0]);
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }
}
