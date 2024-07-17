use std::collections::BinaryHeap;

// https://leetcode.com/problems/the-kth-factor-of-n/description/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

struct Solution;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut priority_queue = BinaryHeap::new();
        let sqrt = (n as f64).sqrt() as i32 + 1;

        for i in 1..sqrt {
            if n % i == 0 {
                priority_queue.push(i);
                Self::remove_exceeding(&mut priority_queue, k as usize);
                if (n / i) != i {
                    priority_queue.push(n / i);
                    Self::remove_exceeding(&mut priority_queue, k as usize);
                }
            }
        }

        if priority_queue.len() < k as usize {
            -1
        } else {
            priority_queue.pop().unwrap()
        }
    }

    fn remove_exceeding(binary_queue: &mut BinaryHeap<i32>, k: usize) {
        while binary_queue.len() > k {
            binary_queue.pop();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::kthfactorofn::Solution;

    #[test]
    fn test_one() {
        assert_eq!(-1, Solution::kth_factor(4, 4))
    }
}
