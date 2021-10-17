use std::rc::Rc;
use std::cell::RefCell;
use crate::nodes::TreeNode;

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = Vec::new();
        queue.push((root.clone(), 0));
        let mut result: Vec<Vec<i32>> = Vec::new();

        while !queue.is_empty() {
            let (node, level) = queue.pop().unwrap();

            if let Some(p) = node {
                let bor = p.borrow();

                if result.len() < level + 1 {
                    result.push(Vec::new());
                }

                result[level as usize].push(bor.val);

                queue.push((bor.right.clone(), level + 1));
                queue.push((bor.left.clone(), level + 1));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::binarytreeordertraversal::Solution;
    use crate::nodes::TreeNode;

    #[test]
    fn test_one() {
        let node = TreeNode::from(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);

        assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], Solution::level_order(node));
    }
}