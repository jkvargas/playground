struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut closest = i32::max_value();
        let mut answer = 0;

        for i in 0..nums.len() - 1 {
            let mut low = i + 1;
            let mut high = nums.len() - 1;

            while low != high {
                let res = nums[i] + nums[low] + nums[high];
                let differ = (target - res).abs();

                let min = std::cmp::min(closest, differ);
                if min != closest {
                    closest = min;
                    answer = res;
                }

                if res < target {
                    low += 1;
                } else {
                    high -= 1;
                }
            }
        }

        answer
    }
}

// 1, 1, 1, 0 target = 100
// 0, 1, 1, 1
// |  |     |
// i  low   hi
//
// 0 + 1 + 1 = 2
// closer = 2
//
// 2 < 100 so, low++
//
// 0, 1, 1, 1
//    |  |  |
//    i low hi
// 1 + 1 + 1 = 3
// closer = 3 -- min(target - sum, closer)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_sum_closest_1() {
        let result = Solution::three_sum_closest(vec![-1, 2, 1, -4], 1);

        assert_eq!(2, result);
    }

    #[test]
    fn three_sum_closest_2() {
        let result = Solution::three_sum_closest(vec![1, 1, 1, 0], 100);

        assert_eq!(3, result);
    }

    #[test]
    fn three_sum_closest_3() {
        let result = Solution::three_sum_closest(vec![1, 1, -1, -1, 3], -1);

        assert_eq!(-1, result);
    }

    #[test]
    fn three_sum_closest_4() {
        let result = Solution::three_sum_closest(vec![0, 2, 1, -3], 0);

        assert_eq!(0, result);
    }
}
