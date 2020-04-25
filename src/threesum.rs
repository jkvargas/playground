pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() {
            return Vec::<Vec::<i32>>::new();
        }

        nums.sort();

        let mut result = Vec::<Vec::<i32>>::new();

        for i in 0..(nums.len() - 2) {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut start = i + 1;
            let mut end = nums.len() - 1;

            while start < end {
                let sum = nums[i] + nums[start] + nums[end];

                if sum > 0 {
                    end -= 1;
                    continue;
                }

                if sum < 0 {
                    start += 1;
                    continue;
                }

                if sum == 0 {
                    result.push(vec![nums[i], nums[start], nums[end]]);

                    while start + 1 < end && nums[start] == nums[start+1] {
                        start += 1;
                    }

                    while end - 1 > start && nums[end] == nums[end-1] {
                        end -= 1;
                    }

                    start += 1;
                    end -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_1() {
        let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);

        assert_eq!(result, vec![vec![-1, 0, 1], vec![-1, -1, 2]]);
    }

    #[test]
    fn three_sum_2() {
        let result = Solution::three_sum(vec![0, 0, 0, 0]);

        assert_eq!(result, vec![vec![0, 0, 0]]);
    }
}
