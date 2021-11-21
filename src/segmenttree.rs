use std::{cell::RefCell, collections::HashMap, rc::Rc};

type OptionalNode = Option<Rc<RefCell<SegmentNode>>>;

#[derive(PartialEq, Eq, Clone, Debug)]
struct SegmentNode {
    val: i32,
    right: OptionalNode,
    left: OptionalNode,
    parent: OptionalNode,
}

pub struct SegmentTree {
    segment_node: OptionalNode,
    leaf_node_map: HashMap<usize, OptionalNode>,
    range_map: HashMap<(usize, usize), OptionalNode>,
}

trait CalculatesMiddle {
    fn calculate_middle(start_range: usize, end_range: usize) -> usize {
        let middle = (end_range as f32 - start_range as f32) / 2.0;
        let abs_mid = middle.abs();

        if middle - abs_mid > 0.0 {
            1 as usize
        } else {
            abs_mid as usize
        }
    }
}

impl CalculatesMiddle for SegmentTree {}

impl CalculatesMiddle for SegmentNode {}

impl SegmentTree {
    pub fn new(vec: &Vec<i32>) -> SegmentTree {
        let mut leaf_node_map = HashMap::new();
        let mut range_map = HashMap::new();
        let segment_node = SegmentNode::new(&vec, &mut leaf_node_map, &mut range_map);

        Self {
            range_map,
            segment_node,
            leaf_node_map,
        }
    }

    pub fn query(&self, start_range: usize, end_range: usize) -> i32 {
        if start_range < 0 || end_range > self.range_map.len() {
            return 0;
        }

        if let Some(contains) = self.range_map.get(&(start_range, end_range)) {
            if let Some(node) = contains {
                return node.as_ref().borrow().val;
            }
        }

        let middle_point = start_range + Self::calculate_middle(start_range, end_range);

        self.query(start_range, middle_point) + self.query(middle_point + 1, end_range)
    }

    pub fn update(&mut self, index: usize, val: i32) {
        if let Some(lnode) = self.leaf_node_map.get_mut(&index) {
            let node = lnode.as_mut().unwrap();
            let mut borrow = node.borrow_mut();

            let mut to_update = val - borrow.val;
            borrow.val = val;

            let mut parent_ref = borrow.parent.clone();

            while let Some(pref_unwrap) = parent_ref {
                let mut pref_borrow = pref_unwrap.borrow_mut();

                let hold_previous = pref_borrow.val;
                pref_borrow.val += to_update;
                to_update = pref_borrow.val - hold_previous;

                parent_ref = pref_borrow.parent.clone();
            }
        }
    }
}

impl SegmentNode {
    fn new(
        vec: &Vec<i32>,
        leaf_map: &mut HashMap<usize, OptionalNode>,
        range_map: &mut HashMap<(usize, usize), OptionalNode>,
    ) -> OptionalNode {
        let vec_len = vec.len();
        Self::build_tree(&vec, 0, vec_len - 1, leaf_map, range_map)
    }

    fn build_tree(
        vec: &Vec<i32>,
        start_range: usize,
        end_range: usize,
        leaf_map: &mut HashMap<usize, OptionalNode>,
        range_map: &mut HashMap<(usize, usize), OptionalNode>,
    ) -> OptionalNode {
        if start_range == end_range {
            let node = Some(Rc::new(RefCell::new(Self {
                parent: None,
                val: vec[start_range],
                right: None,
                left: None,
            })));

            leaf_map.insert(start_range, node.clone());
            range_map.insert((start_range, end_range), node.clone());

            return node;
        }

        let mid = start_range + Self::calculate_middle(start_range, end_range);

        let left_side = Self::build_tree(vec, start_range, mid, leaf_map, range_map);
        let right_side = Self::build_tree(vec, mid + 1, end_range, leaf_map, range_map);

        let sum = Self::sum_sides(left_side.as_ref(), right_side.as_ref());

        let node = Some(Rc::new(RefCell::new(Self {
            parent: None,
            val: sum,
            right: right_side.clone(),
            left: left_side.clone(),
        })));

        left_side.as_ref().unwrap().borrow_mut().parent = node.clone();
        right_side.as_ref().unwrap().borrow_mut().parent = node.clone();

        range_map.insert((start_range, end_range), node.clone());

        node
    }

    fn sum_sides(
        left: Option<&Rc<RefCell<SegmentNode>>>,
        right: Option<&Rc<RefCell<SegmentNode>>>,
    ) -> i32 {
        let ls = left.unwrap();
        let rs = right.unwrap();

        let ls_borrow = ls.borrow();
        let rs_borrow = rs.borrow();

        ls_borrow.val + rs_borrow.val
    }
}

#[cfg(test)]
mod when_dealing_with_segment_tree {
    use crate::segmenttree::{SegmentNode, SegmentTree};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn get_range_after_update() {
        let vec: Vec<i32> = vec![1, 3, -2, 8, -7];

        let mut node_tree = SegmentTree::new(&vec);

        node_tree.update(2, 4);

        assert_eq!(8, node_tree.query(0, 2));
    }

    #[test]
    fn query_with_range_split() {
        let vec: Vec<i32> = vec![1, 3, 5, 7, 9, 11];

        let mut node_tree = SegmentTree::new(&vec);

        assert_eq!(16, node_tree.query(0, 3));
    }

    #[test]
    fn updating_an_item_correctly() {
        let vec: Vec<i32> = vec![1, 3, -2, 8, -7];

        let mut node_tree = SegmentTree::new(&vec);

        node_tree.update(2, 4);

        let root = node_tree.segment_node.as_ref().unwrap();
        let root_borrow = root.borrow();

        assert_eq!(9, root_borrow.val);

        let left = root_borrow.left.as_ref().unwrap();
        let left_borrow = left.borrow();

        assert_eq!(8, left_borrow.val);

        let right = left_borrow.right.as_ref().unwrap();
        let right_borrow = right.borrow();

        assert_eq!(4, right_borrow.val);
    }

    #[test]
    fn tree_is_built_correctly() {
        let vec: Vec<i32> = vec![1, 3, -2, 8, -7];

        let node_tree = SegmentTree::new(&vec);
        let node_object = node_tree.segment_node.unwrap();
        let node = node_object.borrow();

        assert!(node.parent.is_none());
        assert_eq!(3, node.val);

        let first_left = node.left.as_ref().unwrap();
        let first_right = node.right.as_ref().unwrap();

        let first_left_borrow = first_left.borrow();
        assert_eq!(2, first_left_borrow.val);

        let first_right_borrow = first_right.borrow();
        assert_eq!(1, first_right_borrow.val);

        let sr_r = first_right_borrow.right.as_ref().unwrap();
        let sr_l = first_right_borrow.left.as_ref().unwrap();

        let sr_r_borrow = sr_r.borrow();
        let sr_l_borrow = sr_l.borrow();

        assert_eq!(-7, sr_r_borrow.val);
        assert_eq!(8, sr_l_borrow.val);

        let sl_r = first_left_borrow.right.as_ref().unwrap();
        let sl_l = first_left_borrow.left.as_ref().unwrap();

        let sl_r_borrow = sl_r.borrow();
        assert_eq!(-2, sl_r_borrow.val);

        let sl_l_borrow = sl_l.borrow();
        assert_eq!(4, sl_l_borrow.val);

        let t_r = sl_l_borrow.right.as_ref().unwrap();
        let t_l = sl_l_borrow.left.as_ref().unwrap();

        let t_r_borrow = t_r.borrow();
        let t_l_borrow = t_l.borrow();

        assert_eq!(1, t_l_borrow.val);
        assert_eq!(3, t_r_borrow.val);
    }

    #[test]
    fn parent_of_node_must_match() {
        let vec: Vec<i32> = vec![1, 3, -2, 8, -7];

        let node_tree = SegmentTree::new(&vec);
        let node_object = node_tree.segment_node.unwrap();
        let node = node_object.borrow();

        let node_val = node.val;

        let left = node.left.as_ref().unwrap();
        let right = node.right.as_ref().unwrap();

        let left_borrow = left.borrow();
        let right_borrow = right.borrow();

        let left_parent = left_borrow.parent.as_ref().unwrap();
        let right_parent = right_borrow.parent.as_ref().unwrap();

        assert_eq!(node_val, left_parent.borrow().val);
        assert_eq!(node_val, right_parent.borrow().val);
    }
}
