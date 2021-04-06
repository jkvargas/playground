struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() < 2 {
            return;
        }

        let mut i = nums.len() - 2;

        while i > 0 && nums[i + 1] <= nums[i] {
            i -= 1;
        }

        if nums[i + 1] > nums[i] {
            let mut j = nums.len() - 1;
            while j >= 0 && nums[j] <= nums[i] {
                j -= 1;
            }

            Self::swap(nums, i, j);

            Self::reverse(nums, i + 1);
        } else {
            Self::reverse(nums, i);
        }
    }

    fn reverse(vec: &mut Vec<i32>, start: usize) {
        let mut i = start;
        let mut j = vec.len() - 1;

        while i < j {
            Self::swap(vec, i, j);
            i += 1;
            j -= 1;
        }
    }

    fn swap(vec: &mut Vec<i32>, i: usize, j: usize) {
        let temp = vec[i];
        vec[i] = vec[j];
        vec[j] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_permutation_1() {
        let mut vec = vec![1, 2, 3];

        Solution::next_permutation(&mut vec);

        assert_eq!(vec![1, 3, 2], vec);
    }

    #[test]
    fn next_permutation_2() {
        let mut vec = vec![3, 2, 1];

        Solution::next_permutation(&mut vec);

        assert_eq!(vec![1, 2, 3], vec);
    }

    #[test]
    fn next_permutation_3() {
        let mut vec = vec![1, 1, 5];

        Solution::next_permutation(&mut vec);

        assert_eq!(vec![1, 5, 1], vec);
    }

    #[test]
    fn next_permutation_4() {
        let mut vec = vec![1, 3, 2];

        Solution::next_permutation(&mut vec);

        assert_eq!(vec![2, 1, 3], vec);
    }

    #[test]
    fn next_permutation_5() {
        let mut vec = vec![1, 5, 8, 4, 7, 6, 5, 3, 1];

        Solution::next_permutation(&mut vec);

        assert_eq!(vec![1, 5, 8, 5, 1, 3, 4, 6, 7], vec);
    }
}
