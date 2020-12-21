use crate::nodes::TreeNode;

use std::ops::Add;
use std::{
    cell::{Ref, RefCell},
    collections::VecDeque,
    rc::Rc,
    str::FromStr,
};

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

type TreeNodeOption = Option<Rc<RefCell<TreeNode>>>;

pub struct Codec {
    queue: VecDeque<TreeNodeOption>,
    vec_str: Vec<String>,
}

impl Codec {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            vec_str: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.queue.clear();
        self.vec_str.clear();
    }

    fn serialize(&mut self, root: TreeNodeOption) -> String {
        self.clear();

        self.queue.push_back(root);
        let mut last_number = 0;

        while !self.queue.is_empty() {
            let node = self.queue.pop_front().unwrap();

            match node {
                None => self.vec_str.push("None".to_string()),
                Some(content) => {
                    let borrow = content.borrow();
                    self.vec_str.push(borrow.val.to_string());
                    last_number = self.vec_str.len();
                    self.queue.push_back(borrow.left.clone());
                    self.queue.push_back(borrow.right.clone());
                }
            }
        }

        self.vec_str.truncate(last_number);
        self.vec_str.join("|")
    }

    fn deserialize(&self, data: String) -> TreeNodeOption {
        let temp: Vec<Option<i32>> = data
            .split("|")
            .map(|x| {
                if x == "None" {
                    None
                } else {
                    Some(FromStr::from_str(x).unwrap())
                }
            })
            .collect();

        Self::get_node_from_numbers(0, &temp)
    }

    fn get_node_from_numbers(index: usize, data: &Vec<Option<i32>>) -> TreeNodeOption {
        if let Some(val) = data[index] {
            let mut node = Some(Rc::new(RefCell::new(TreeNode::new(val))));

            if index * 2 + 1 < data.len() {
                if let Some(inherited) = &node {
                    let mut borrow = inherited.borrow_mut();
                    borrow.left = Self::get_node_from_numbers(index * 2 + 1, data);
                }
            }

            if index * 2 + 2 < data.len() {
                if let Some(inherited) = &node {
                    let mut borrow = inherited.borrow_mut();
                    borrow.right = Self::get_node_from_numbers(index * 2 + 2, data);
                }
            }

            return node;
        }

        None
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::TreeNode;

    #[test]
    fn simplify_path_1() {
        let to_serialize = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
        ]);

        let expect = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
        ]);

        let mut obj = Codec::new();
        let data = obj.serialize(to_serialize);

        let serialized = "1|2|3|None|None|4|5".to_string();
        assert_eq!(serialized, data);

        let ans = obj.deserialize(data);

        assert_eq!(ans, expect);
    }
}
