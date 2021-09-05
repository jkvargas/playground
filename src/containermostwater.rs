use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut begin = 0;
        let mut end = height.len() - 1;
        let mut max_result = 0;

        while begin != end {
            let max = (end - begin) as i32 * std::cmp::min(height[begin], height[end]);

            if max > max_result {
                max_result = max;
            }

            if height[begin] > height[end] {
                end -= 1;
            } else {
                begin += 1;
            }
        }

        max_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_area_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn max_area_2() {
        assert_eq!(Solution::max_area(vec![1, 8]), 1);
    }

    #[test]
    fn max_area_3() {
        assert_eq!(Solution::max_area(vec![3, 6, 9, 12, 4, 15, 0, 4, 6]), 42);
    }

    #[test]
    fn max_area_4() {
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }

    #[test]
    fn max_area_5() {
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }
}
