use crate::sortingalgos::heapsort::heap_sort;

pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        heap_sort(nums);
    }
}


#[cfg(test)]
mod test {
    use crate::sortcolors::Solution;

    // Input: nums = [2,0,2,1,1,0]
    // Output: [0,0,1,1,2,2]
    #[test]
    fn test_one() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut input);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], input);
    }

    // Input: nums = [2,0,1]
    // Output: [0,1,2]
    #[test]
    fn test_two() {
        let mut input = vec![2, 0, 1];
        Solution::sort_colors(&mut input);
        assert_eq!(vec![0, 1, 2], input);
    }
}