struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::with_capacity(intervals.len() + 1);
        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            answer.push(intervals[i].clone());
            i += 1;
        }
        while i < intervals.len() && intervals[i][0] <= new_interval[1] {
            new_interval[0] = std::cmp::min(new_interval[0], intervals[i][0]);
            new_interval[1] = std::cmp::max(new_interval[1], intervals[i][1]);
            i += 1;
        }
        answer.push(new_interval);
        while i < intervals.len() {
            answer.push(intervals[i].clone());
            i += 1;
        }
        answer
    }
}

#[cfg(test)]
mod test {
    use crate::insertinterval::Solution;

    #[test]
    fn test_one() {
        let result: Vec<Vec<i32>> = vec![vec![1, 5], vec![6, 9]];
        let parameter: Vec<Vec<i32>> = vec![vec![1, 3], vec![6, 9]];
        let argument = vec![2, 5];
        assert_eq!(result, Solution::insert(parameter, argument));
    }

    #[test]
    fn test_two() {
        let result: Vec<Vec<i32>> = vec![vec![1, 3], vec![4, 10]];
        let parameter: Vec<Vec<i32>> = vec![vec![1, 3], vec![7, 10]];
        let argument = vec![4, 8];
        assert_eq!(result, Solution::insert(parameter, argument));
    }

    #[test]
    fn test_three() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4], vec![5, 6]],
            Solution::insert(vec![vec![1, 2], vec![3, 4]], vec![5, 6])
        );
    }

    #[test]
    fn test_four() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::insert(vec![vec![1, 5]], vec![2, 3])
        );
    }
}
