use std::cmp::min;

struct Solution;

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let mut min = 1;
        let mut max =  10000000;
        let mut min_speed = -1;

        while min <= max {
            let mid = (min + max) / 2;
            let mut time = 0.0;
            for i in 0..dist.len() {
                let t = dist[i] as f64 / mid as f64;
                time += if i == dist.len() - 1 { t } else { t.ceil() };
            }

            if time <= hour {
                max = mid - 1;
                min_speed = mid;
            } else {
                min = mid + 1;
            }
        }

        min_speed
    }
}

#[cfg(test)]
mod test {
    use crate::minimum_speed_to_arrive_on_time::Solution;

    #[test]
    fn test_one() {
        assert_eq!(3, Solution::min_speed_on_time(vec![1, 3, 2], 2.7));
    }

    #[test]
    fn test_two() {
        assert_eq!(1, Solution::min_speed_on_time(vec![1, 3, 2], 6.0));
    }
}
