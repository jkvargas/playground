use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            *map.entry(nums[i]).or_insert(0usize) += 1usize;
        }

        let mut bh = BinaryHeap::new();
        map.iter().map(|(&num, &count)| Occurrence { n: num, o: count }).for_each(|ocurrence| bh.push(ocurrence));

        let mut result = Vec::new();
        (0..k as usize).for_each(|i| { result.push( bh.pop().unwrap().n); });
        result
    }
}

struct Occurrence {
    n: i32,
    o: usize,
}

impl Eq for Occurrence {}

impl PartialEq<Self> for Occurrence {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.o == other.o
    }
}

impl PartialOrd<Self> for Occurrence {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.o.cmp(&other.o))
    }
}

impl Ord for Occurrence {
    fn cmp(&self, other: &Self) -> Ordering {
        self.o.cmp(&other.o)
    }
}

#[cfg(test)]
mod tests {
    use crate::top_k_frequent::Solution;

    #[test]
    fn it_works() {
        assert_eq!(vec![1,2], Solution::top_k_frequent(vec![1,1,1,2,2,3], 2));
    }
}