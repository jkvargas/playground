use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, i32>::new();
        map.insert(0, -1);

        let mut max_length = 0;
        let mut count = 0;

        for (i, val) in nums.iter().enumerate() {
            count = count + if *val == 1 {
                1
            } else {
                -1
            };

            if map.contains_key(&count) {
                max_length = std::cmp::max(max_length, (i as i32) - map.get(&count).unwrap());
            } else {
                map.insert(count, i as i32);
            }
        }

        max_length as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_length_1() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
    }

    #[test]
    fn find_max_length_2() {
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }

    #[test]
    fn find_max_length_4() {
        assert_eq!(Solution::find_max_length(vec![0,1,0,1]), 4);
    }

    #[test]
    fn find_max_length_3() {
        assert_eq!(Solution::find_max_length(vec![0, 0, 1, 0, 0, 0, 1, 1]), 6);
    }
}
