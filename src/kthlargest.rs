pub struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        nums[nums.len() - k as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn find_kth_largest_1() {
        let result = vec![3,2,3,1,2,4,5,5,6];

        assert_eq!(Solution::find_kth_largest(result, 4), 4);
    }
}