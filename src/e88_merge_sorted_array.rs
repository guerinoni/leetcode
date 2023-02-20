pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut idx1 = m - 1;
    let mut idx2 = n - 1;
    let mut k = m + n - 1;
    while idx1 >= 0 && idx2 >= 0 {
        if nums1[idx1 as usize] > nums2[idx2 as usize] {
            nums1[k as usize] = nums1[idx1 as usize];
            idx1 -= 1;
        } else {
            nums1[k as usize] = nums2[idx2 as usize];
            idx2 -= 1;
        }
        k -= 1;
    }

    while idx1 >= 0 && idx1 != k {
        nums1[k as usize] = nums1[idx1 as usize];
        k -= 1;
        idx1 -= 1;
    }

    while idx2 >= 0 {
        nums1[k as usize] = nums2[idx2 as usize];
        k -= 1;
        idx2 -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        let expected = vec![1, 2, 2, 3, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(expected, nums1);

        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        let expected = vec![1];
        merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(expected, nums1);

        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        let expected = vec![1];
        merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(expected, nums1);

        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        let expected = vec![1, 2, 3, 4, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(expected, nums1);
    }
}
