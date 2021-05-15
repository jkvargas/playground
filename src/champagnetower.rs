// vz(row, glass) = (vz(row - 1, glass - 1) + vz(row - 1, glass) - 1) * 0.5
// am(row, glass) = min(1, 1 - vz(row - 1, glass - 1) + vz(row - 1, glass))

use std::collections::HashMap;

struct Solution;

type Amount = f64;
type FlowRate = f64;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut map = HashMap::new();
        Self::f(query_row, query_glass, poured as f64, &mut map).0
    }

    fn f(
        row: i32,
        glass: i32,
        val_total: f64,
        flow_rate_map: &mut HashMap<(i32, i32), f64>,
    ) -> (Amount, FlowRate) {
        let component = if row == 0 {
            val_total
        } else {
            let first = if glass - 1 < 0 {
                0.0
            } else {
                if flow_rate_map.contains_key(&(row - 1, glass - 1)) {
                    *flow_rate_map.get(&(row - 1, glass - 1)).unwrap()
                } else {
                    Self::f(row - 1, glass - 1, val_total, flow_rate_map).1
                }
            };

            let size_of_row = row - 1;
            let second = if glass <= size_of_row {
                if flow_rate_map.contains_key(&(row - 1, glass)) {
                    *flow_rate_map.get(&(row - 1, glass)).unwrap()
                } else {
                    Self::f(row - 1, glass, val_total, flow_rate_map).1
                }
            } else {
                0.0
            };

            first + second
        };

        let amount = if component > 1.0 { 1.0 } else { component };
        let flow_rate = (component - 1.0) * 0.5;
        let flow_res = if flow_rate >= 0.0 { flow_rate } else { 0.0 };

        flow_rate_map.insert((row, glass), flow_res);

        (amount, flow_res)
    }
}

#[test]
pub fn test1() {
    assert_eq!(0.0, Solution::champagne_tower(1, 1, 1));
}

#[test]
pub fn test2() {
    assert_eq!(0.5, Solution::champagne_tower(2, 1, 1));
}

#[test]
pub fn test3() {
    assert_eq!(1.0, Solution::champagne_tower(100000009, 33, 17));
}
