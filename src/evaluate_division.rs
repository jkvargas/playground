// https://leetcode.com/problems/evaluate-division/description/?envType=study-plan-v2&envId=top-interview-150

use std::collections::{HashMap, HashSet};

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

        queries.iter().for_each(|query| result.push(if let Some(from_calc) = calc(&query[0], &query[1], &map, &mut HashSet::new()) {
            from_calc
        } else {
            -1.0
        }));
        
        result
    }
}

fn calc(a: &String, b: &String, map: &HashMap<String, HashMap<String, f64>>, visited: &mut HashSet<String>) -> Option<f64> {
    if !map.contains_key(a) { return None; }
    if !map.contains_key(b) { return None; }
    if visited.contains(a) { return None; } else { visited.insert(a.clone()); }
    if a == b { return Some(1.0); }
    let from_key = map.get(a).unwrap();

    if from_key.contains_key(b) {
        Some(*from_key.get(b).unwrap())
    } else {
        let mut result = None;

        for key in from_key.keys() {
            if let Some(nested) = calc(key, b, map, visited) {
                result = Some(*from_key.get(key).unwrap() * nested);
                break;
            }
        }

        result
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
