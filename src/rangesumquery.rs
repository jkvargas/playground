// https://leetcode.com/problems/range-sum-query-mutable/

use crate::segmenttree::SegmentTree;

struct NumArray {
    segment_tree: SegmentTree,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            segment_tree: SegmentTree::new(&nums),
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.segment_tree.update(index as usize, val);
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.segment_tree.query(left as usize, right as usize)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
#[cfg(test)]
mod config {
    use crate::rangesumquery::NumArray;

    #[test]
    fn test_one() {
        let mut num_array = NumArray::new(vec![1, 3, 5]);

        assert_eq!(9, num_array.sum_range(0, 2));

        num_array.update(1, 2);

        assert_eq!(8, num_array.sum_range(0, 2));
    }

    #[test]
    fn test_sec() {
        let mut num_array = NumArray::new(vec![0, 9, 5, 7, 3]);

        num_array.sum_range(4, 4);
        num_array.sum_range(2, 4);
        num_array.sum_range(3, 3);
        num_array.update(4, 5);
        num_array.update(1, 7);
        num_array.update(0, 8);
        num_array.sum_range(1, 2);
        num_array.update(1, 2);
        num_array.sum_range(4, 4);
        num_array.update(3, 4);
    }
}
