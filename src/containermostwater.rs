use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_area(mut height: Vec<i32>) -> i32 {
        let mut map : HashMap<i32, Vec<usize>> = HashMap::new();

        for i in 0..height.len() {
            if map.contains_key(&height[i]) {
                map.insert(height[i], Vec::new());
            }
            map.get_mut(&height[i]).unwrap().push(i);
        }

        height.sort();

        let mut highest = 0;
        let mut lowest = 0;

        for i in height.reverse() {

        }

        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_area_1() {
        assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }

    #[test]
    fn max_area_2() {
        assert_eq!(Solution::max_area(vec![1,8]), 1);
    }

    #[test]
    fn max_area_3() {
        assert_eq!(Solution::max_area(vec![3,6,9,12,4,15,0,4,6]), 42);
    }
}