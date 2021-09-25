use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::cmp::min;
use std::ops::Add;
use std::rc::Rc;

struct Solution;

struct FoundNode {
    found_letters: u8,
    node: i32,
    lca: i32,
}

impl FoundNode {
    fn not_found() -> FoundNode {
        FoundNode {
            found_letters: 0,
            node: 0,
            lca: 0,
        }
    }

    fn new(found_p: bool, found_q: bool) -> FoundNode {
        let mut found_letters = 0;

        if found_p {
            found_letters |= 0x1;
        }

        if found_q {
            found_letters |= 0x2;
        }

        FoundNode {
            lca: 0,
            node: 0,
            found_letters,
        }
    }

    #[inline(always)]
    fn found_both(&self) -> bool {
        self.found_letters & 0x3 == 3
    }
}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let res = Self::lca_from(root, p.unwrap().borrow().val, q.unwrap().borrow().val, 0);

        Some(Rc::new(RefCell::new(TreeNode::new(res.unwrap().node))))
    }

    fn lca_from(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
        count: i32,
    ) -> Option<FoundNode> {
        if root.is_none() {
            return None;
        }

        let r_unr = root.unwrap();
        let borrow = r_unr.borrow();

        let next = count + 1;
        let from_left = Self::lca_from(borrow.left.clone(), p, q, next);
        let from_right = Self::lca_from(borrow.right.clone(), p, q, next);

        let found_p = borrow.val == p;
        let found_q = borrow.val == q;

        if from_left.is_none() && from_right.is_none() && !found_q && !found_p {
            return None;
        }

        let node = if found_q || found_p {
            Some(FoundNode::new(found_p, found_q))
        } else {
            None
        };

        let from_children = Self::sum_up(from_left, from_right, count, borrow.val);
        let result = Self::sum_up(from_children, node, count, borrow.val);

        result
    }

    fn sum_up(
        mut left: Option<FoundNode>,
        mut right: Option<FoundNode>,
        count: i32,
        parent: i32,
    ) -> Option<FoundNode> {
        if left.is_none() && right.is_none() {
            return None;
        }

        if left.is_none() ^ right.is_none() {
            return if left.is_none() { right } else { left };
        }

        let mut from_left = left.unwrap();
        let from_right = right.unwrap();

        if from_left.found_both() && from_right.found_both() {
            return if from_left.lca > from_right.lca {
                Some(from_right)
            } else {
                Some(from_left)
            };
        }

        if from_left.found_both() ^ from_right.found_both() {
            return if from_left.found_both() {
                Some(from_left)
            } else {
                Some(from_right)
            };
        }

        from_left.found_letters |= from_right.found_letters;

        if from_left.found_both() {
            from_left.lca = count;
            from_left.node = parent;
        }

        Some(from_left)
    }
}

/// Fastest
/// enum DFSResult {
//     Answer(Rc<RefCell<TreeNode>>),
//     FoundNode(Rc<RefCell<TreeNode>>),
//     NotFound,
// }
//
// impl Solution {
//     pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//         let p = p.unwrap().borrow().val;
//         let q = q.unwrap().borrow().val;
//
//         match Solution::dfs(root, p, q) {
//             DFSResult::Answer(answer) => Some(answer),
//             DFSResult::FoundNode(_) => panic!(),
//             DFSResult::NotFound => panic!()
//         }
//     }
//
//     fn dfs(node: Option<Rc<RefCell<TreeNode>>>, first: i32, second: i32) -> DFSResult {
//         match node {
//             None => DFSResult::NotFound,
//             Some(node) => {
//                 let val = node.borrow().val;
//                 if val == first || val == second {
//                     // If we want the second node in left or right then current node is the parent
//                     if let DFSResult::FoundNode(_) = Solution::dfs(node.borrow().left.clone(), first, second) {
//                         return DFSResult::Answer(node.clone());
//                     }
//
//                     if let DFSResult::FoundNode(_) = Solution::dfs(node.borrow().right.clone(), first, second) {
//                         return DFSResult::Answer(node.clone());
//                     }
//
//                     DFSResult::FoundNode(node.clone())
//                 } else {
//                     match Solution::dfs(node.borrow().left.clone(), first, second) {
//                         DFSResult::Answer(node) => DFSResult::Answer(node),
//                         DFSResult::FoundNode(_) => {
//                             match Solution::dfs(node.borrow().right.clone(), first, second) {
//                                 DFSResult::Answer(_) => panic!(),
//                                 DFSResult::FoundNode(_) => DFSResult::Answer(node.clone()),
//                                 DFSResult::NotFound => DFSResult::FoundNode(node.clone()),
//                             }
//                         }
//                         DFSResult::NotFound => {
//                             Solution::dfs(node.borrow().right.clone(), first, second)
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

#[test]
fn test_one() {
    let bt = TreeNode::from(vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);

    let res = Solution::lowest_common_ancestor(
        bt,
        Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        Some(Rc::new(RefCell::new(TreeNode::new(1)))),
    );

    assert_eq!(3, res.unwrap().borrow().val);
}
