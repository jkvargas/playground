// https://leetcode.com/problems/different-ways-to-add-parentheses/?envType=problem-list-v2&envId=dynamic-programming

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut memo = HashMap::new();

        calc(&expression, &mut memo)
    }
}

fn calc(expression: &String, memo: &mut HashMap<String, Vec<i32>>) -> Vec<i32>{
    if memo.contains_key(expression) {
        return memo[expression].clone();
    }

    let mut results = Vec::new();
    if expression.len() == 0 { return results; }

    let expr = expression.chars().collect::<Vec<char>>();

    if expr.iter().all(|x| x.is_digit(10)) {
        results.push(expr[0].to_string().parse::<i32>().unwrap());
        return results;
    }

    if expr.len() == 2 && expr[0].is_digit(10) {
        results.push(expr[0].to_string().parse::<i32>().unwrap());
        return results;
    }

    for i in 0..expr.len() {
        let current = expr[i];
        if current.is_digit(10) {
            continue;
        }

        let left = calc(&expr[0..i].iter().collect::<String>(), memo);
        let right = calc(&expr[i+1..expr.len()].iter().collect::<String>(), memo);

        for lv in &left {
            for rv in &right {
                let mut computed = 0;

                match current {
                    '+' => {
                        computed = lv + rv;
                    },
                    '-' => {
                        computed = lv - rv;
                    },
                    '*' => {
                        computed = lv * rv;
                    },
                    _ => {}
                }

                results.push(computed);
            }
        }
    }

    memo.insert(expression.clone(), results.clone());
    results
}


#[cfg(test)]
mod tests {
    use crate::different_ways_to_parentheses::Solution;

    #[test]
    fn test_one() {
        let vec = Solution::diff_ways_to_compute("2-1-1".to_string());
        assert_eq!(vec![2, 0], vec);
    }
}