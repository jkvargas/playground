use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;

struct SolutionJho;

struct Window {
    start: usize,
    end: usize,
    array: Vec<i32>,
    heap: BTreeMap<usize, i32>,
}

impl Window {
    pub fn new(size: usize, array: Vec<i32>) -> Self {
        let mut heap = BTreeMap::new();

        for i in 0..size {
            heap.insert(i, array[i]);
        }

        dbg!(&heap);

        Self {
            start: 0,
            end: size - 1,
            array,
            heap,
        }
    }

    pub fn max(&self) -> i32 {
        *self.heap.values().max().unwrap()
    }

    pub fn consume(&mut self) -> bool {
        if self.end < self.array.len() - 1 {
            self.heap.remove(&self.start);
            self.start = self.start + 1;
            self.end = self.end + 1;
            self.heap.insert(self.end, self.array[self.end]);
            return true;
        }

        false
    }
}

impl SolutionJho {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut window = Window::new(k as usize, nums);
        let mut result = Vec::new();

        loop {
            result.push(window.max());

            if !window.consume() {
                break;
            }
        }

        result
    }
}

struct SecondSolution;

impl SecondSolution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut window = VecDeque::new();

        for i in 0..k as usize {
            window.push_back(nums[i])
        }

        vec![]
    }
}

struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut res: Vec<i32> = Vec::with_capacity(n - k + 1);
        let mut q = VecDeque::new();
        for i in 0..n {
            while let Some(&id) = q.front() {
                if id + k <= i {
                    q.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&id) = q.back() {
                if nums[id] <= nums[i] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(i);
            if i >= k - 1 {
                if let Some(&id) = q.front() {
                    res.push(nums[id]);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_sliding_window_1() {
        let result = Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);

        assert_eq!(vec![3, 3, 5, 5, 6, 7], result);
    }

    #[test]
    fn max_sliding_window_2() {
        let result = Solution::max_sliding_window(vec![1, -1], 1);

        assert_eq!(vec![1, -1], result);
    }

    #[test]
    fn max_sliding_window_3() {
        let result = Solution::max_sliding_window(vec![-7, -8, 7, 5, 7, 1, 6, 0], 4);

        assert_eq!(vec![7, 7, 7, 7, 7], result);
    }
}
