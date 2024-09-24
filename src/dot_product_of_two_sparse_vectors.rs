// https://leetcode.com/problems/dot-product-of-two-sparse-vectors/description/

use std::collections::HashMap;

struct SparseVector {
    map: HashMap<usize, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut map = HashMap::new();

        nums.iter().enumerate().for_each(|(ind, &val)| if val != 0 { map.insert(ind, val); });

        Self {
            map
        }
    }

    // Return the dotProduct of two sparse vectors
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut sum = 0;
        self.map.keys().for_each(|pos| {
            if let Some(&val) = vec.map.get(pos) {
                sum += val * *self.map.get(pos).unwrap()
            }
        });

        sum
    }
}

/**
 * Your SparseVector object will be instantiated and called as such:
 * let v1 = SparseVector::new(nums1);
 * let v2 = SparseVector::new(nums2);
 * let ans = v1.dot_product(v2);
 */