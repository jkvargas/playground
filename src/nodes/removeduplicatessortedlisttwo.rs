use crate::nodes::ListNode;
use std::collections::{BTreeSet, HashSet};

struct Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut number = BTreeSet::new();
        let mut repeated = HashSet::new();

        while let Some(ln) = head {
            if number.contains(&ln.val) {
                repeated.insert(ln.val);
                number.remove(&ln.val);
            } else {
                if !repeated.contains(&ln.val) {
                    number.insert(ln.val);
                }
            }

            head = ln.next;
        }

        let mut next = None;

        for &i in number.iter().rev() {
            let mut node = ListNode::new(i);
            node.next = next;
            next = Some(Box::new(node));
        }

        next
    }
}

#[cfg(test)]
mod tests {
    use crate::nodes::removeduplicatessortedlisttwo::Solution;
    use crate::nodes::ListNode;

    #[test]
    fn test_one() {
        assert_eq!(
            vec![1, 2, 5],
            ListNode::flat_it(Solution::delete_duplicates(ListNode::from(vec![
                1, 2, 3, 3, 4, 4, 5
            ])))
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(
            vec![1, 2, 3, 6, 7],
            ListNode::flat_it(Solution::delete_duplicates(ListNode::from(vec![
                4, 6, 3, 4, 1, 2, 7
            ])))
        );
    }

    #[test]
    fn test_three() {
        assert_eq!(
            vec![2, 3],
            ListNode::flat_it(Solution::delete_duplicates(ListNode::from(vec![
                1, 1, 1, 2, 3
            ])))
        );
    }
}
