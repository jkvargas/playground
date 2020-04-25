pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;

        while start <= end {
            let mid = start + ((end - start + 1) >> 1);

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[mid] > nums[start] {
                if target >= nums[start] && target <= nums[mid] {
                    if mid > 0 {
                        end = mid - 1;
                    } else {
                        break;
                    }
                } else {
                    start = mid + 1;
                }
            } else {
                if target <= nums[end] && target > nums[mid] {
                    start = mid + 1;
                } else {
                    if mid > 0 {
                        end = mid - 1;
                    } else {
                        break;
                    }
                }
            }
        }

        -1
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_4() {
        assert_eq!(Solution::search(vec![1, 3], 1), 0);
    }

    #[test]
    fn search_5() {
        assert_eq!(Solution::search(vec![1, 3], 0), -1);
    }

    #[test]
    fn search_7() {
        assert_eq!(Solution::search(vec![1, 3], 2), -1);
    }

    #[test]
    fn search_6() {
        assert_eq!(Solution::search(vec![3, 1], 3), 0);
    }

    #[test]
    fn search_3() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }

    #[test]
    fn search_1() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
    }

    #[test]
    fn search_2() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
    }

    #[test]
    fn search_8() {
        assert_eq!(Solution::search(vec![1, 3, 5], 4), -1);
    }

    #[test]
    fn search_9() {
        assert_eq!(Solution::search(vec![8, 9, 2, 3, 4], 9), 1);
    }

    #[test]
    fn search_10() {
        assert_eq!(Solution::search(vec![5, 1, 3], 3), 2);
    }

}