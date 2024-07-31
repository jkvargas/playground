// https://leetcode.com/problems/find-minimum-time-to-finish-all-jobs-ii/description/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

struct Solution;

impl Solution {
    pub fn minimum_time(mut jobs: Vec<i32>, mut workers: Vec<i32>) -> i32 {
        jobs.sort();
        workers.sort();

        let mut max = i32::MIN;

        for ind in 0..jobs.len() {
            let i = jobs[ind];
            let j = workers[ind];
            let mut val = i / j;
            if i % j != 0 {
                val += 1;
            }

            max = max.max(val);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::find_minimum_time_to_finish_all_jobs_ii::Solution;

    #[test]
    fn test_one() {
        assert_eq!(2, Solution::minimum_time(vec![5, 2, 4], vec![1, 7, 5]));
    }
}
