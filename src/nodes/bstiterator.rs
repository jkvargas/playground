use crate::nodes::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
struct BSTIterator {
    index: Option<usize>,
    vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut flattened_bt = Vec::new();

        Self::flatten_it(root.as_ref(), &mut flattened_bt);

        BSTIterator {
            index: None,
            vec: flattened_bt,
        }
    }

    fn flatten_it(root: Option<&Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(r) = root {
            let bor = r.borrow();

            Self::flatten_it(bor.left.as_ref(), vec);
            vec.push(bor.val);
            Self::flatten_it(bor.right.as_ref(), vec);
        }
    }

    fn next(&mut self) -> i32 {
        let mut value = 0;

        if self.index.is_some() {
            let ind = self.index.unwrap();
            self.index = Some(ind + 1);
            value = self.vec[ind + 1];
        } else {
            value = self.vec[0];
            self.index = Some(0);
        }

        value
    }

    fn has_next(&self) -> bool {
        if self.vec.len() > 0 {
            if let Some(x) = self.index {
                if x < self.vec.len() - 1 {
                    return true;
                }
            } else {
                return true;
            }
        }

        false
    }
}

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[test]
fn test_one() {
    let mut tn = TreeNode::from(vec![
        Some(7),
        Some(3),
        Some(15),
        None,
        None,
        Some(9),
        Some(20),
    ]);

    let mut bsi = BSTIterator::new(tn);

    assert_eq!(3, bsi.next());
    assert_eq!(7, bsi.next());
    assert!(bsi.has_next());
    assert_eq!(9, bsi.next());
    assert!(bsi.has_next());
    assert_eq!(15, bsi.next());
    assert!(bsi.has_next());
    assert_eq!(20, bsi.next());
    assert_eq!(false, bsi.has_next());
}
