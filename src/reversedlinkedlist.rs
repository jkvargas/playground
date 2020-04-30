struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;
        while let Some(mut node) = head {
            head = node.next;
            node.next = new_head;
            new_head = Some(node);
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_list_1() {
        let result = Solution::reverse_list(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: None
                        }))
                    }))
                }))
            }))
        })));

        assert_eq!(result, Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 1,
                            next: None
                        }))
                    }))
                }))
            }))
        })));
    }
}