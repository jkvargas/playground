struct Solution;

impl Solution {
    pub fn missing_element(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut last = nums[0];
        let mut result = 0;

        for i in 1..nums.len() {
            let n = nums[i];
            let how_many = n - last - 1;
            if how_many >= k {
                result = last + k;
                k = 0;
                break;
            } else {
                k -= how_many;
            }

            last = n;
        }

        if k > 0 {
            result = last + k;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::missing_elements_in_sorted_array::Solution;

    #[test]
    fn test_one() {
        assert_eq!(5, Solution::missing_element(vec![4,7,9,10], 1));
    }

    #[test]
    fn two() {
        assert_eq!(6, Solution::missing_element(vec![1,2,4], 3));
    }

    #[test]
    fn three() {
        assert_eq!(8, Solution::missing_element(vec![4,7,9,10], 3));
    }
}