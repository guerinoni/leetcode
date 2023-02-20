// NOTE: short iteration with 2 index.
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut j = nums.len() - 1;
    let mut i = 0;
    loop {
        if nums[i] == val {
            loop {
                if nums[j] != val {
                    break;
                }

                if j == 0 || j - 1 <= i {
                    return i.try_into().unwrap();
                }

                j -= 1;
            }

            nums.swap(i, j);
        }

        if j <= i {
            break;
        }

        i += 1;
    }

    (i + 1).try_into().unwrap()
}

// NOTE: easier to read but iteration is longer.
// pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
//     let mut j = 0;
//     for i in 0..nums.len() {
//         if nums[i] != val {
//             nums.swap(i, j);
//             j += 1;
//         }
//     }

//     j.try_into().unwrap()
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2);
        assert_eq!(remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
        assert_eq!(remove_element(&mut vec![], 0), 0);
        assert_eq!(remove_element(&mut vec![0, 0, 0, 0, 0, 0, 0, 0], 0), 0);
        assert_eq!(remove_element(&mut vec![0, 0, 0, 0, 0, 0, 0, 0], 1), 8);
        assert_eq!(remove_element(&mut vec![1, 2], 1), 1);
        assert_eq!(remove_element(&mut vec![1], 1), 0);
    }
}
