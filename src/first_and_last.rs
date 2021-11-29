// https://leetcode.com/explore/interview/card/google/63/sorting-and-searching-4/3081/

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let first_pos = Self::lower_bound(&nums, target);
        let last_pos = Self::upper_bound(&nums, target);

        if first_pos > last_pos {
            return vec![-1, -1];
        }

        vec![first_pos, last_pos]
    }

    fn lower_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut lo: usize = 0;
        let mut hi: usize = nums.len();

        while (lo < hi) {
            let mid: usize = lo + (hi - lo) / 2;

            if target > nums[mid] {
                lo = 1 + mid;
            } else {
                hi = mid;
            }
        }

        lo as i32
    }

    fn upper_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut lo: usize = 0;
        let mut hi: usize = nums.len();

        while (lo < hi) {
            let mid: usize = lo + (hi - lo) / 2;

            if target < nums[mid] {
                hi = mid;
            } else {
                lo = 1 + mid;
            }
        }

        hi as i32 - 1
    }
}

#[cfg(test)]
mod test {
    use crate::first_and_last::Solution;

    #[test]
    fn test_one() {
        let result = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8);

        assert_eq!(vec![3, 4], result);
    }

    #[test]
    fn test_sec() {
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }

    #[test]
    fn test_three() {
        assert_eq!(vec![-1, -1], Solution::search_range(vec![], 0));
    }

    #[test]
    fn test_four() {
        assert_eq!(vec![0, 0], Solution::search_range(vec![1], 1));
    }
}
