struct Solution;

impl Solution {
    pub fn min_cost_ii(mut costs: Vec<Vec<i32>>) -> i32 {
        for i in (0..costs.len() - 1).rev() {
            for j in 0..costs[0].len() {
                let mut min_price = i32::MAX;

                for s in 0..costs[0].len() {
                    if j == s {
                        continue;
                    }

                    let new_cost = costs[i][j] + costs[i + 1][s];

                    if new_cost < min_price {
                        min_price = new_cost;
                    }
                }

                costs[i][j] = min_price;
            }
        }

        let mut result = i32::MAX;
        for i in &costs[0] {
            if *i < result {
                result = *i;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::painthouseii::Solution;

    #[test]
    pub fn test_one() {
        let mut mat = vec![vec![1, 5, 3], vec![2, 9, 4]];

        let res = Solution::min_cost_ii(mat);

        assert_eq!(5, res);
    }
}
