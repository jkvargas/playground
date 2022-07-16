use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

struct Solution;

struct Worker {
    quality: i32,
    wage: i32,
    ratio: f64,
}

impl Worker {
    fn new(quality: i32, wage: i32) -> Self {
        let ratio = wage as f64 / quality as f64;
        Self {
            quality,
            wage,
            ratio,
        }
    }
}

impl Eq for Worker {}

impl PartialEq<Self> for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.wage == other.wage &&
            self.quality == other.quality &&
            self.ratio == other.ratio
    }
}

impl PartialOrd<Self> for Worker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ratio > other.ratio {
            Ordering::Greater
        } else {
            if self.ratio < other.ratio {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }
    }
}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        Self::f(0, &quality, &wage, None, k).unwrap()
    }

    fn f(i: usize, quality: &Vec<i32>, wage: &Vec<i32>, base_ratio: Option<f64>, k: i32) -> Option<f64> {
        if i >= quality.len() {
            if k > 0 {
                return None;
            } else {
                return Some(0.0);
            }
        }

        if k == 0 {
            return Some(0.0);
        }

        let ratio_to_use = if let Some(br) = base_ratio {
            br
        } else {
            wage[i] as f64 / quality[i] as f64
        };

        let base_salary = ratio_to_use * quality[i] as f64;

        let result = if base_salary >= wage[i] as f64 {
            if quality.len() - (i + 1) >= k as usize {
                let with = Self::f(i + 1, quality, wage, Some(ratio_to_use), k - 1)
                    .map_or(None, |x| Some(base_salary + x));
                let without = Self::f(i + 1, quality, wage, base_ratio, k);

                if with < without {
                    with
                } else {
                    without
                }
            } else {
                Self::f(i + 1, quality, wage, Some(ratio_to_use), k - 1)
                    .map_or(None, |x| Some(base_salary + x))
            }
        } else {
            Self::f(i + 1, quality, wage, base_ratio, k)
        };

        result
    }

    pub fn mincost_to_hire_workers_leetcode(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut workers = Vec::new();
        for i in 0..quality.len() {
            let worker = Worker::new(quality[i], wage[i]);
            workers.push(worker);
        }

        workers.sort();

        let mut ans = f64::MAX;
        let mut heap = BinaryHeap::new();
        let mut sum = 0.0;
        for worker in workers {
            heap.push(worker.quality);
            sum += worker.quality as f64;

            if heap.len() > k as usize {
                sum -= heap.pop().unwrap() as f64;
            }

            if heap.len() == k as usize {
                let temp = sum * worker.ratio;
                if temp < ans {
                    ans = temp;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::minimal_cost_to_hire_k_workers::Solution;

    #[test]
    fn test_one() {
        assert_eq!(105.0, Solution::mincost_to_hire_workers_leetcode(vec![10, 20, 5], vec![70, 50, 30], 2));
    }

    #[test]
    fn test_two() {
        assert_eq!(30.666666666666664, Solution::mincost_to_hire_workers_leetcode(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3));
    }
}