struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ignore = vec![0, (nums.len() - 1) as i32];

        match Self::try_binary_search(&nums, 0, nums.len() as i32 - 1, target, &mut ignore) {
            None => vec![-1, -1],
            Some(mut last_found) => {
                let mut first_found = last_found;
                let end= nums.len() - 1;

                while let Some(new_search) = Self::try_binary_search(&nums, last_found + 1, end as i32, target, &mut ignore) {
                    last_found = new_search;
                }

                while let Some(new_search) = Self::try_binary_search(&nums, 0, first_found - 1, target, &mut ignore) {
                    first_found = new_search;
                }

                vec![first_found, last_found]
            }
        }
    }

    fn try_binary_search(nums: &Vec<i32>, mut start: i32, mut end: i32, target: i32, ignore: &mut Vec<i32>) -> Option<i32> {
        if start < 0 {
            return None;
        }

        if end as usize >= nums.len() {
            return None;
        }

        start = std::cmp::max(start, ignore[0]);
        end = std::cmp::min(end, ignore[1]);

        while start <= end {
            let mid_point = (start as usize + end as usize) >> 1;

            if nums[mid_point] < target {
                ignore[0] = mid_point as i32;
                start = mid_point as i32 + 1;
            } else if nums[mid_point] > target {
                ignore[1] = mid_point as i32;
                end = mid_point as i32 - 1;
            } else {
                return Some(mid_point as i32);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn search_range_1() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    }

    #[test]
    pub fn search_range_2() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }
}