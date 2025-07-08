use std::os::linux::raw::stat;

struct Solution;

impl Solution {
    pub fn isBadVersion(&self, version:i32)-> bool {
        version == 2
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut end = n;
        let mut start = 1;
        while start < end {
            let mid = (end - start) / 2 + start;
            if self.isBadVersion(mid) {
                end = mid;
            } else {
                start = mid + 1;
            }
        }

        start
    }
}

#[cfg(test)]
mod tests {
    use crate::first_bad_version::Solution;

    #[test]
    fn test_two() {
        let sol = Solution {};

        assert_eq!(2, sol.first_bad_version(2));
    }

    #[test]
    fn test_one() {
        let sol = Solution {};

        assert_eq!(4, sol.first_bad_version(5));
    }
}