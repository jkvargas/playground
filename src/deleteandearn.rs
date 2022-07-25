use std::cmp::max;
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut memo = HashMap::new();

        for num in &nums {
            let present = map.get(num).map_or(0, |x| *x);
            map.insert(*num, present + *num);
        }

        let mut sorted_unique_numbers: Vec<i32> = map.keys().map(|x| x.clone()).collect();
        sorted_unique_numbers.sort();

        Self::take(&sorted_unique_numbers, &map, 0, &mut memo)
    }

    fn take(
        sorted_unique_numbers: &Vec<i32>,
        map: &HashMap<i32, i32>,
        i: usize,
        memo: &mut HashMap<usize, i32>,
    ) -> i32 {
        if i >= sorted_unique_numbers.len() {
            return 0;
        }

        let n = sorted_unique_numbers[i];

        if i + 1 >= sorted_unique_numbers.len() {
            return *map.get(&n).unwrap();
        }

        if let Some(res) = memo.get(&i) {
            return *res;
        }

        if i + 1 >= sorted_unique_numbers.len() {
            return *map.get(&n).unwrap();
        }

        let mut with_take = i + 1;
        while sorted_unique_numbers[with_take] < n + 2 {
            with_take += 1;
        }

        let mut without_take = i + 1;

        let max_value = max(
            map.get(&n).unwrap() + Self::take(sorted_unique_numbers, map, with_take, memo),
            Self::take(sorted_unique_numbers, map, without_take, memo),
        );

        memo.insert(i, max_value);

        max_value
    }

    pub fn delete_and_earn_leetcode(nums: Vec<i32>) -> i32 {
        let mut points = HashMap::new();
        let mut cache = HashMap::new();

        let mut max_number = 0;
        for num in &nums {
            let present = points.get(num).map_or(0, |x| *x);
            points.insert(*num, present + *num);
            max_number = max(max_number, *num);
        }

        Self::max_points(max_number, &points, &mut cache)
    }

    fn max_points(num: i32, points: &HashMap<i32, i32>, cache: &mut HashMap<i32, i32>) -> i32 {
        if num == 0 {
            return 0;
        }

        if num == 1 {
            return points.get(&num).map_or(0, |x| *x);
        }

        if cache.contains_key(&num) {
            return *cache.get(&num).unwrap();
        }

        let gain = points.get(&num).map_or(0, |x| *x);
        let without = Self::max_points(num - 1, points, cache);
        let with = Self::max_points(num - 2, points, cache);
        cache.insert(num, max(without, with + gain));

        return *cache.get(&num).unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::deleteandearn::Solution;

    #[test]
    fn test_one() {
        assert_eq!(
            9,
            Solution::delete_and_earn_leetcode(vec![2, 2, 3, 3, 3, 4])
        );
    }
}
