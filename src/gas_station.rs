struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let total_gas = gas.iter().fold(0, |acc, g| acc + g);
        let total_cost = cost.iter().fold(0, |acc, x| acc + x);
        if total_gas < total_cost { return -1; }

        let mut start = 0;
        let mut last_gas = 0;

        for i in 0..gas.len() {
            last_gas += gas[i] - cost[i];

            if last_gas < 0 {
                start = i + 1;
                last_gas = 0;
            }
        }

        start as i32
    }
}
#[cfg(test)]
mod tests {
    use crate::gas_station::Solution;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(3, Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]));
    }
}

