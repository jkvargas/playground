struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {

    }
}

fn dfs(starting_point: usize, gas: &Vec<i32>, cost: &Vec<i32>) {
    dfs(starting_point + 1, gas, cost);
}

#[cfg(test)]
mod tests {
    use crate::gas_station::Solution;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(3, Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]));
    }
}

