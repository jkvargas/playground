struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type OptionListNode = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l = l1.unwrap_or(Box::new(ListNode::new(0)));
        let mut r = l2.unwrap_or(Box::new(ListNode::new(0)));
        let (mut cur_node_val, mut add) = Self::get_sum_from_nodes(&l, &r, 0);
        let mut node = Some(Box::new(ListNode::new(cur_node_val)));
        let mut node_ref = &mut node;
        let mut l_not_found = false;
        let mut r_not_found = false;

        loop {
            l = l.next.unwrap_or_else(|| {
                l_not_found = true;
                Box::new(ListNode::new(0))
            });
            r = r.next.unwrap_or_else(|| {
                r_not_found = true;
                Box::new(ListNode::new(0))
            });

            if r_not_found && l_not_found && add == 0 {
                break;
            }

            let (next_cur, next_add) = Self::get_sum_from_nodes(&l, &r, add);
            cur_node_val = next_cur;
            add = next_add;

            if let Some(content) = node_ref {
                content.next = Some(Box::new(ListNode::new(cur_node_val)));
                node_ref = &mut content.next;
            }
        }

        node
    }

    fn get_sum_from_nodes(l: &Box<ListNode>, r: &Box<ListNode>, mut add: i32) -> (i32, i32) {
        let mut cur_node_val = l.val + r.val + add;

        if cur_node_val == 0 {
            return (0, add);
        }

        if cur_node_val > 9 {
            add = cur_node_val / 10;
            cur_node_val = cur_node_val % 10;
        } else {
            add = 0;
        }

        (cur_node_val, add)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers_1() {
        let l1 = build_from_numbers(vec![2, 4, 3]);
        let l2 = build_from_numbers(vec![5, 6, 4]);

        let result = Solution::add_two_numbers(l1, l2).unwrap();

        let expected = build_from_numbers(vec![7, 0, 8]).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_two_numbers_2() {
        let l1 = build_from_numbers(vec![0]);
        let l2 = build_from_numbers(vec![0]);

        let result = Solution::add_two_numbers(l1, l2).unwrap();

        let expected = build_from_numbers(vec![0]).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_two_numbers_3() {
        let l1 = build_from_numbers(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = build_from_numbers(vec![9, 9, 9, 9]);

        let result = Solution::add_two_numbers(l1, l2).unwrap();

        let expected = build_from_numbers(vec![8, 9, 9, 9, 0, 0, 0, 1]).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_two_numbers_4() {
        let l1 = build_from_numbers(vec![2, 4, 9]);
        let l2 = build_from_numbers(vec![5, 6, 4, 9]);

        let result = Solution::add_two_numbers(l1, l2).unwrap();

        let expected = build_from_numbers(vec![7, 0, 4, 0, 1]).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_two_numbers_5() {
        let l1 = build_from_numbers(vec![9]);
        let l2 = build_from_numbers(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]);

        let result = Solution::add_two_numbers(l1, l2).unwrap();

        let expected = build_from_numbers(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn add_two_numbers_6() {
        let l1 = build_from_numbers(vec![1, 6, 0, 3, 3, 6, 7, 2, 0, 1]);
        let l2 = build_from_numbers(vec![6, 3, 0, 8, 9, 6, 6, 9, 6, 1]);

        let result = Solution::add_two_numbers(l1, l2).unwrap();

        let expected = build_from_numbers(vec![7, 9, 0, 1, 3, 3, 4, 2, 7, 2]).unwrap();

        assert_eq!(result, expected);
    }

    fn build_from_numbers(vec: Vec<i32>) -> OptionListNode {
        let mut listnode = Some(Box::new(ListNode::new(vec[0])));

        let mut node_ref = &mut listnode;
        for i in 1..vec.len() {
            let new_node = Box::new(ListNode::new(vec[i]));

            if let Some(content) = node_ref {
                content.next = Some(new_node);
                node_ref = &mut content.next;
            }
        }

        listnode
    }
}
