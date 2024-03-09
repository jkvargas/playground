struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut j = 1;
        let mut count = 1;

        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                count += 1;
            } else {
                count = 1;
            }

            if count <= 2 {
                nums[j] = nums[i];
                j += 1;
            }
        }

        j as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_duplicates_from_sorted_array_2::Solution;

    #[test]
    fn test_one() {
        let mut content = vec![1, 1, 1, 2, 2, 3];

        assert_eq!(5, Solution::remove_duplicates(&mut content));
        assert_eq!(vec![1, 1, 2, 2, 3], content);
    }

    #[test]
    fn test_two() {
        let mut content = vec![1, 1, 1];

        assert_eq!(2, Solution::remove_duplicates(&mut content));
        assert_eq!(vec![1, 1], content);
    }

    #[test]
    fn test_three() {
        let mut content = vec![1, 1, 1, 1];

        assert_eq!(2, Solution::remove_duplicates(&mut content));
        assert_eq!(vec![1, 1], content);
    }
}
