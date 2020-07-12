use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;
use std::borrow::Borrow;
use crate::nodes::TreeNode;

struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        if !root.is_some() {
            return result;
        }

        let mut queue = VecDeque::new();

        queue.push_back((root.unwrap(), 0));

        while !queue.is_empty() {
            let (node, lvl) = queue.pop_front().unwrap();
            let borrowed: &RefCell<TreeNode> = node.borrow();
            let val = borrowed.borrow();
            let right = val.right.clone();
            let left = val.left.clone();

            if result.len() < lvl + 1 {
                result.push(Vec::new());
            }

            result[lvl].push(val.val);

            if let Some(l) = left {
                if (lvl + 1) % 2 == 0 {
                    queue.push_back((l, lvl + 1));
                } else {
                    queue.push_front((l, lvl + 1));
                }
            }

            if let Some(r) = right {
                if (lvl + 1) % 2 == 0 {
                    queue.push_back((r, lvl + 1));
                } else {
                    queue.push_front((r, lvl + 1));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn zigzag_level_order_1()
    {
        let tree_node = TreeNode::from(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::zigzag_level_order(tree_node), vec![vec![3], vec![20, 9], vec![15, 7]]);
    }

    #[test]
    pub fn zigzag_level_order_2()
    {
        let tree_node = TreeNode::from(vec![Some(1), Some(2), Some(3), Some(4), None, None, Some(5)]);
        assert_eq!(Solution::zigzag_level_order(tree_node), vec![vec![1], vec![3, 2], vec![5, 4]]);
    }
}