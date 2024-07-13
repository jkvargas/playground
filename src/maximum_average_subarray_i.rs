struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        let mut max = 0;

        for i in 0..k {
            sum += nums[i as usize];
        }

        max = sum;

        for i in k as usize..nums.len() {
            sum -= nums[i - k as usize];
            sum += nums[i];

            if sum > max {
                max = sum
            }
        }

        max as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::maximum_average_subarray_i::Solution;

    #[test]
    fn test_one() {
        assert_eq!(12.7500, Solution::find_max_average(vec![1,12,-5,-6,50,3], 4));
    }

    #[test]
    fn test_two() {
        assert_eq!(5.0, Solution::find_max_average(vec![5], 1));
    }

    #[test]
    fn test_three() {
        assert_eq!(4.0, Solution::find_max_average(vec![0,4,0,3,2], 1))
    }
}