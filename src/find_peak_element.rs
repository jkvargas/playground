// https://leetcode.com/explore/interview/card/facebook/54/sorting-and-searching-3/3032/

struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        
        while l < r {
            let m = (l + r) / 2;
            if nums[m] > nums[m + 1] {
                r = m;
            } else {
                l = m + 1;
            }
        }
        
        l as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::find_peak_element::Solution;

    #[test]
    fn test_one() {
        assert_eq!(2, Solution::find_peak_element(vec![1,2,3,1]));
    }

    #[test]
    fn test_two() {
        assert_eq!(5, Solution::find_peak_element(vec![1,2,1,3,5,6,4]));
    }
}