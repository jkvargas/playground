// https://leetcode.com/problems/evaluate-division/description/?envType=study-plan-v2&envId=top-interview-150

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut map: HashMap<String, HashMap<String, f64>> = HashMap::new();

        for i in 0..equations.len() {
            map.entry(equations[i][0].clone()).or_insert(HashMap::new()).entry(equations[i][1].clone()).or_insert(values[i]);
            map.entry(equations[i][1].clone()).or_insert(HashMap::new()).entry(equations[i][0].clone()).or_insert(1. / values[i]);
        }

        let mut result = Vec::new();
        
        for i in 0..queries.len() {
            
        }
        
        result
    }
}

fn calc(starting_value: f64, element: &String, map: &HashMap<String, HashMap<String, f64>>) -> Option<f64> {
    if map.contains_key(element) {
        return starting_value * map.get(element).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate_division::Solution;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
            Solution::calc_equation(
                vec![
                    vec!["a".to_string(), "b".to_string()],
                    vec!["b".to_string(), "c".to_string()]
                ],
                vec![2.0, 3.0],
                vec![
                    vec!["a".to_string(), "c".to_string()],
                    vec!["b".to_string(), "a".to_string()],
                    vec!["a".to_string(), "e".to_string()],
                    vec!["a".to_string(), "a".to_string()],
                    vec!["x".to_string(), "x".to_string()]
                ]
            )
        );
    }
}
