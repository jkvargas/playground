// https://leetcode.com/problems/paint-fence/description/?envType=study-plan-v2&envId=premium-algo-100

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        let mut memo = HashMap::new();
        total_ways(&mut memo, n, k)
    }
}

fn total_ways(memo: &mut HashMap<i32, i32>, i: i32, k: i32) -> i32 {
    if i == 0 {
        return 0;
    }

    if i == 1 {
        return k;
    }

    if i == 2 {
        return k * k;
    }

    if memo.contains_key(&i) {
        return *memo.get(&i).unwrap();
    }

    let minus_one = total_ways(memo, i - 1, k);
    let minus_two = total_ways(memo, i - 2, k);

    memo.insert(i, (k - 1) * (minus_one + minus_two));
    *memo.get(&i).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::paint_fence::Solution;

    #[test]
    fn test_one() {
        assert_eq!(6, Solution::num_ways(3, 2));
    }
}
