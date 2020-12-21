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

        while !self.queue.is_empty() {
            let node = self.queue.pop_front().unwrap();

            match node {
                None => self.vec_str.push("None".to_string()),
                Some(content) => {
                    let borrow = content.borrow();
                    self.vec_str.push(borrow.val.to_string());
                    self.queue.push_back(borrow.left.clone());
                    self.queue.push_back(borrow.right.clone());
                }
            }
        }

        self.vec_str.join("|")
    }

    fn deserialize(&self, data: String) -> TreeNodeOption {
        if data.is_empty() {
            return None;
        }

        let lst: Vec<Option<i32>> = data
            .split("|")
            .map(|x| {
                if x == "None" {
                    None
                } else {
                    Some(x.parse::<i32>().unwrap())
                }
            })
            .collect();

        let root = match lst[0] {
            Some(a) => Rc::new(RefCell::new(TreeNode::new(a))),
            None => return None,
        };

        let mut cur = 1;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        while let Some(node) = queue.pop_front() {
            if let Some(a) = lst[cur] {
                let l = Rc::new(RefCell::new(TreeNode::new(a)));
                node.borrow_mut().left = Some(l.clone());
                queue.push_back(l);
            }
            cur += 1;
            if let Some(a) = lst[cur] {
                let r = Rc::new(RefCell::new(TreeNode::new(a)));
                node.borrow_mut().right = Some(r.clone());
                queue.push_back(r);
            }
            cur += 1;
        }
        Some(root)
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
    fn path_2() {
        let to_serialize = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);

        dbg!(&to_serialize);

        let expect = TreeNode::from(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
            Some(6),
            Some(7),
        ]);

        let mut obj = Codec::new();
        let data = obj.serialize(to_serialize);

        dbg!(data);
    }

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
