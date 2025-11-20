// https://leetcode.com/problems/delete-and-earn/description/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut points: HashMap<i32, i32> = HashMap::new();
        let mut max_number = 0;

        // Precompute how many points we gain from taking an element
        for &num in &nums {
            *points.entry(num).or_insert(0) += num;
            max_number = max_number.max(num);
        }

        // DP array
        let mut max_points = vec![0; (max_number + 1) as usize];
        if max_number >= 1 {
            max_points[1] = *points.get(&1).unwrap_or(&0);
        }

        for num in 2..=max_number {
            let gain = *points.get(&num).unwrap_or(&0);
            max_points[num as usize] =
                max_points[(num - 1) as usize].max(max_points[(num - 2) as usize] + gain);
        }

        max_points[max_number as usize]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one() {
        let result = super::Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]);
        assert_eq!(result, 9);
    }
}
