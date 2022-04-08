// NOTE: first approach.
// pub fn my_sqrt(x: i32) -> i32 {
//     for i in 0..=x {
//         let current = i * i;
//         if current == x {
//             return i;
//         } else if current > x {
//             return i - 1;
//         }
//     }

//     0
// }

use std::cmp::Ordering;

pub fn my_sqrt(x: i32) -> i32 {
    if x == 1 {
        return 1;
    }

    let mut low = 1;
    let mut high = x / 2;
    let mut ret = 0;
    while low <= high {
        let middle = low + (high - low) / 2;
        let to_compare = x / middle;
        match middle.cmp(&to_compare) {
            Ordering::Equal => return middle,
            Ordering::Less => {
                low = middle + 1;
                ret = middle
            }
            Ordering::Greater => high = middle - 1,
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(my_sqrt(9), 3);
        assert_eq!(my_sqrt(8), 2);
        assert_eq!(my_sqrt(4), 2);
        assert_eq!(my_sqrt(85), 9);
        assert_eq!(my_sqrt(100), 10);
        assert_eq!(my_sqrt(1), 1);
        assert_eq!(my_sqrt(0), 0);
        assert_eq!(my_sqrt(2147483647), 46340);
    }
}
