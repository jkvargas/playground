struct Solution;
struct SolutionTwo;

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

use std::collections::VecDeque;

impl SolutionTwo {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut window: VecDeque<Box<ListNode>> = VecDeque::with_capacity((n + 1) as usize);
        let mut new_head = None;
        let mut new_node = &mut new_head;

        while let Some(mut node) = head {
            head = node.next.take();
            window.push_back(node);

            if window.len() > n as usize {
                let front = window.pop_front().unwrap();
                *new_node = Some(front);
                new_node = &mut new_node.as_mut().unwrap().next;
            }
        }

        window.pop_front();

        for node in window {
            *new_node = Some(node);
            new_node = &mut new_node.as_mut().unwrap().next;
        }

        new_head
    }
}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
        Self::remove(head, n).1
    }

    fn remove(node: Option<Box<ListNode>>, n: usize) -> (usize, Option<Box<ListNode>>) {
        if let Some(unr) = node {
            let (mut tail_index, tail) = Self::remove(unr.next, n);
            tail_index += 1;

            if tail_index == n {
                return (tail_index, tail);
            } else {
                return (
                    tail_index,
                    Some(Box::new(ListNode {
                        val: unr.val,
                        next: tail,
                    })),
                );
            }
        }

        (0, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_from(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut nodes: Vec<ListNode> = vec.into_iter().map(|x| ListNode::new(x)).collect();

        loop {
            let node = nodes.pop().unwrap();

            if nodes.is_empty() {
                return Some(Box::new(node));
            } else {
                nodes.last_mut().unwrap().next = Some(Box::new(node));
            }
        }
    }

    #[test]
    fn remove_nth_from_end_1() {
        let result = Solution::remove_nth_from_end(build_from(vec![1, 2, 3, 4, 5]), 2);

        assert_eq!(result, build_from(vec![1, 2, 3, 5]));
    }

    #[test]
    fn remove_nth_from_end_2() {
        let result = Solution::remove_nth_from_end(build_from(vec![1, 2, 3, 4, 5]), 1);

        assert_eq!(result, build_from(vec![1, 2, 3, 4]));
    }
}
