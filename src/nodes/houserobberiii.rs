use crate::nodes::TreeNode;
use std::{
    cell::RefCell,
    cmp::max,
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut map = HashMap::new();

        Self::f(root.as_ref(), false, &mut map)
    }

    fn hash_it<T>(obj: T) -> u64
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        hasher.finish()
    }

    fn f(
        r: Option<&Rc<RefCell<TreeNode>>>,
        parent_robbed: bool,
        m: &mut HashMap<(u64, bool), i32>,
    ) -> i32 {
        if let Some(node) = r {
            let bor = node.borrow();
            let hash = Self::hash_it(bor.deref());

            if m.contains_key(&(hash, parent_robbed)) {
                return *m.get(&(hash, parent_robbed)).unwrap();
            }

            if parent_robbed {
                return Self::f(bor.right.as_ref(), false, m)
                    + Self::f(bor.left.as_ref(), false, m);
            } else {
                let rob = bor.val
                    + Self::f(bor.right.as_ref(), true, m)
                    + Self::f(bor.left.as_ref(), true, m);
                let not_rob =
                    Self::f(bor.right.as_ref(), false, m) + Self::f(bor.left.as_ref(), false, m);

                let res = max(rob, not_rob);

                m.insert((hash, parent_robbed), res);

                return res;
            }
        }

        0
    }
}

#[test]
fn test_one() {
    assert_eq!(
        7,
        Solution::rob(TreeNode::from(vec![
            Some(3),
            Some(2),
            Some(3),
            None,
            Some(3),
            None,
            Some(1)
        ]))
    );
}

#[test]
fn test_two() {
    assert_eq!(
        9,
        Solution::rob(TreeNode::from(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(3),
            None,
            Some(1)
        ]))
    );
}
