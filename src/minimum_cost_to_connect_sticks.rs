use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn connect_sticks(mut sticks: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();

        for i in sticks {
            heap.push(Reverse(i));
        }

        let mut total = 0;

        while heap.len() != 1 {
            let a = heap.pop().unwrap();
            let b = heap.pop().unwrap();

            let sum = a.0 + b.0;
            total += sum;

            heap.push(Reverse(sum));
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_cost_to_connect_sticks::Solution;

    #[test]
    fn test_one() {
        assert_eq!(14, Solution::connect_sticks(vec![2,4,3]));
    }
}