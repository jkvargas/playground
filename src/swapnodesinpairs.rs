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

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head.as_ref();
        let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut reference = &mut result;

        while let Some(unw) = &head {
            if let Some(next) = &unw.next {
                reference.as_mut().unwrap().next = Some(Box::new(ListNode::new(next.val)));
                reference = &mut reference.as_mut().unwrap().next;
                reference.as_mut().unwrap().next = Some(Box::new(ListNode::new(unw.val)));
                reference = &mut reference.as_mut().unwrap().next;

                head = next.next.as_ref();
            } else {
                reference.as_mut().unwrap().next = Some(Box::new(ListNode::new(unw.val)));
                reference = &mut reference.as_mut().unwrap().next;

                head = None;
            }
        }

        result.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn swap_pairs_1() {
        let mut node_a = Box::new(ListNode::new(1));
        let mut node_b = Box::new(ListNode::new(2));
        let mut node_c = Box::new(ListNode::new(3));
        let node_d = Box::new(ListNode::new(4));

        node_c.next = Some(node_d);
        node_b.next = Some(node_c);
        node_a.next = Some(node_b);

        let mut result = Solution::swap_pairs(Some(node_a));
        let mut nums: VecDeque<i32> = VecDeque::new();
        nums.push_back(2);
        nums.push_back(1);
        nums.push_back(4);
        nums.push_back(3);

        while !nums.is_empty() {
            let num = nums.pop_front().unwrap();
            let temp = result.unwrap();

            assert_eq!(num, temp.val);

            result = temp.next;
        }
    }
}
