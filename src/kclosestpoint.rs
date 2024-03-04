pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // let mut to_sort = points
        //     .into_iter()
        //     .map(|x| ((x[0] * x[0] + x[1] * x[1]) as u32, x))
        //     .collect::<Vec<(u32, Vec<i32>)>>();
        //
        // to_sort.sort_by(|x, y| x.0.cmp(&y.0));
        //
        // to_sort
        //     .iter()
        //     .take(k as usize)
        //     .map(|x| Vec::<i32>::from((&x.1).as_ref()))
        //     .collect::<Vec<Vec<i32>>>()

        vec![vec![]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn k_closest_1() {
        assert_eq!(
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
            vec![vec![-2, 2]]
        );
    }

    #[test]
    pub fn k_closest_2() {
        assert_eq!(
            Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2),
            vec![vec![3, 3], vec![-2, 4]]
        );
    }
}
