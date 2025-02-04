struct Solution;

impl Solution {
    pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut res: i64 = 0;
        let mut x = nums[0];
        let mut j = 1;
        let mut i = 0;

        while j < n - 1 {
            if nums[i] < nums[j] {
                res += x as i64 * (j - i) as i64;
                i = j;
                x = nums[i];
            }
            j += 1;
        }

        res + x as i64 * (n - 1 - i) as i64
    }
}

#[test]
fn test_one() {
    assert_eq!(7, Solution::find_maximum_score(vec![1, 3, 1, 5]));
}
