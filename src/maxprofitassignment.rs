use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut cost_profit: Vec<(i32, i32)> = Vec::new();

        for i in 0..difficulty.len() {
            cost_profit.push((difficulty[i], profit[i]));
        }

        cost_profit.sort_by(|a, b| (*a).0.cmp(&(*b).0));
        worker.sort();

        let mut result = 0;
        let mut i = 0;
        let mut best = 0;

        for skill in worker {
            while i < difficulty.len() && skill >= cost_profit[i].0 {
                best = std::cmp::max(best, cost_profit[i].1);
                i += 1;
            }
            result += best;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_assignment_1() {
        assert_eq!(
            Solution::max_profit_assignment(
                vec![2, 4, 6, 8, 10],
                vec![10, 20, 30, 40, 50],
                vec![4, 5, 6, 7]
            ),
            100
        );
    }
}
