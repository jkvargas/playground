struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.len() < 2 {
            if nums[0] == target {
                return true;
            } else {
                return false;
            }
        }

        Self::s(&nums, 0, nums.len() - 1, nums[0], target)
    }

    fn s(nums: &Vec<i32>, i: usize, j: usize, initial_value: i32, target: i32) -> bool {
        let middle = ((i as f32) + (j as f32 - i as f32) / 2f32).ceil() as usize;

        if nums[middle] == target {
            return true;
        }

        if nums[i] == target {
            return true;
        }

        if nums[j] == target {
            return true;
        }

        if j - i == 1 {
            return false;
        }

        if nums[middle] < initial_value {
            // we are at the rotated part
            if target > initial_value {
                return Self::s(nums, i, middle, initial_value, target);
            }

            if nums[middle] > target {
                return Self::s(nums, i, middle, initial_value, target);
            } else {
                return Self::s(nums, middle, j, initial_value, target);
            }
        } else {
            // we are at the starting part
            if target < initial_value {
                return Self::s(nums, middle, j, initial_value, target);
            }

            if nums[middle] > target {
                return Self::s(nums, i, middle, initial_value, target);
            } else {
                return Self::s(nums, middle, j, initial_value, target);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::search_in_rotated_sorted_array_2::Solution;

    #[test]
    fn test_one() {
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn test_two() {
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 6));
    }
}
