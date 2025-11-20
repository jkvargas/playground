// https://leetcode.com/explore/featured/card/top-interview-questions-easy/93/linked-list/772/

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use crate::nodes::ListNode;

struct Solution;

impl Solution {
    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut list = Vec::new();

        while let Some(item) = head {
            list.push(item.val);
            head = item.next;
        }

        if list.len() == 1 {
            return true;
        }

        let mut beg: i32 = 0;
        let mut end: i32 = list.len() as i32 - 1;

        loop {
            if list[beg as usize] != list[end as usize] {
                return false;
            }

            beg += 1;
            end -= 1;

            if beg > end {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::palindrome_linked_list::Solution;
    use crate::nodes::ListNode;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(ListNode::from(vec![1, 2, 2, 1])));
    }

    #[test]
    fn test_is_palindrome_sec() {
        assert!(Solution::is_palindrome(ListNode::from(vec![0, 0])));
    }
}
