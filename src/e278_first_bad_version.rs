struct Solution;
impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        version == 4
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut low, mut high) = (1, n);
        while low < high {
            let mid = low + (high - low) / 2;
            if self.isBadVersion(mid) {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let s = Solution;
        assert_eq!(s.first_bad_version(5), 4);
    }
}
