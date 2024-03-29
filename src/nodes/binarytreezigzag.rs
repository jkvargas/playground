use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// struct Solution;
//
// impl Solution {
//     pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
//         let mut result: Vec<Vec<i32>> = Vec::new();
//
//         if root.is_none() {
//             return result;
//         }
//
//         let mut level = VecDeque::new();
//         let mut queue = VecDeque::new();
//
//         queue.push_front(Some(root.unwrap()));
//         queue.push_front(None);
//
//         let mut from_left = true;
//
//         while !queue.is_empty() {
//             if let Some(node) = queue.pop_back().unwrap() {
//                 let borrowed: &RefCell<TreeNode> = node.borrow();
//                 let val = borrowed.borrow();
//
//                 if from_left {
//                     level.push_front(val.val);
//                 } else {
//                     level.push_back(val.val);
//                 }
//
//                 if val.left.is_some() {
//                     queue.push_front(val.left.clone());
//                 }
//                 if val.right.is_some() {
//                     queue.push_front(val.right.clone());
//                 }
//             } else {
//                 let mut vec_result = Vec::new();
//                 while !level.is_empty() {
//                     vec_result.push(level.pop_back().unwrap());
//                 }
//                 result.push(vec_result);
//
//                 level = VecDeque::new();
//
//                 if !queue.is_empty() {
//                     queue.push_front(None);
//                 }
//                 from_left = !from_left;
//             }
//         }
//
//         result
//     }
// }

struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = Vec::new();
        let mut first_time = true;
        let mut result = Vec::new();

        queue.push((root.clone(), 0, true));

        while !queue.is_empty() {
            let (node, level, dir) = queue.pop().unwrap();

            if let Some(n) = node {
                let bor = n.borrow();
                let node_val = bor.val;

                if first_time {
                    result.push(VecDeque::new());
                    result[level].push_front(node_val);
                    first_time = false;
                } else {
                    if result.len() < level + 1 {
                        result.push(VecDeque::new());
                    }

                    if dir {
                        result[level].push_front(node_val);
                    } else {
                        result[level].push_back(node_val);
                    }
                }

                queue.push((bor.left.clone(), level + 1, !dir));
                queue.push((bor.right.clone(), level + 1, !dir));
            }
        }

        result.into_iter().map(|x| x.into()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn zigzag_level_order_1() {
        let tree_node = TreeNode::from(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(
            Solution::zigzag_level_order(tree_node),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }

    #[test]
    pub fn zigzag_level_order_2() {
        let tree_node = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
        ]);
        assert_eq!(
            Solution::zigzag_level_order(tree_node),
            vec![vec![1], vec![3, 2], vec![4, 5]]
        );
    }
}
