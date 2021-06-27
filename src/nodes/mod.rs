use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

mod binarydiameter;
mod binarytreezigzag;
mod bstiterator;
mod houserobberiii;
mod rangesumbst;
mod serializeanddeserializebinarytree;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Hash for TreeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);

        if let Some(l) = self.left.as_ref() {
            let lbor = l.borrow();
            lbor.hash(state);
        } else {
            None::<&TreeNode>.hash(state);
        }

        if let Some(r) = self.right.as_ref() {
            let rbor = r.borrow();
            rbor.hash(state);
        } else {
            None::<&TreeNode>.hash(state);
        }
    }
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::fill(0, &values)
    }

    fn fill(index: usize, values: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(val) = values[index] {
            let mut result = Self::new(val);

            if index * 2 + 1 < values.len() {
                result.left = Self::fill(index * 2 + 1, values);
            }
            if index * 2 + 2 < values.len() {
                result.right = Self::fill(index * 2 + 2, values);
            }

            return Some(Rc::new(RefCell::new(result)));
        }

        None
    }
}
