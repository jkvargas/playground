// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        match nums.binary_search(&target) {
            Ok(found) => {
                let mut end = found;
                while end + 1 < nums.len() {
                    if let Ok(res) = nums[end + 1..nums.len()].binary_search(&target) {
                        end = res;
                    } else {
                        break;
                    }
                }

                let mut beg = found;
                while beg - 1 >= 0 {
                    if let Ok(res) = nums[0..beg].binary_search(&target) {
                        beg = res;
                    } else {
                        break;
                    }
                }

                vec![beg as i32, end as i32]
            }
            Err(_) => {
                vec![-1, -1]
            }
        }
    }
}


#[cfg(test)]
mod find_first_and_last_position_of_element_tests {
    use crate::find_first_and_last_position_of_element::Solution;

    #[test]
    fn find_first_and_last_position_of_element_tests() {
        assert_eq!(vec![3, 4], Solution::search_range(vec![5,7,7,8,8,10], 8));
    }
}

