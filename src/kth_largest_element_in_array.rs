// https://leetcode.com/problems/kth-largest-element-in-an-array/description/

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut binary_heap = BinaryHeap::new();

        for &i in &nums {
            binary_heap.push(i);
        }

        for _ in 0..k - 1 {
            binary_heap.pop();
        }

        binary_heap.pop().unwrap()
    }
}