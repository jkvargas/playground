struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let prev = nums.clone();
        let length = nums.len();
        for i in 0..length {
            nums[((i as i32 + k) % (length as i32)) as usize] = prev[i];
        }
    }

    pub fn rotate_best(nums: &mut Vec<i32>, k: i32) {
        if k == 0 {
            return;
        }
        let n = k as usize % nums.len();
        nums.rotate_right(n);
    }
}

#[cfg(test)]
mod tests {
    use crate::rotate_array::Solution;

    #[test]
    fn test_rotate() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut arr, 3);
        assert_eq!(arr, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_2() {
        let mut arr = vec![-1, -100, 3, 99];
        Solution::rotate(&mut arr, 2);
        assert_eq!(arr, vec![3, 99, -1, -100]);
    }
}
