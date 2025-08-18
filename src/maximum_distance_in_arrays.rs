struct Solution;


impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut max = arrays[0][arrays[0].len() - 1];
        let mut min = arrays[0][0];
        let mut distance = 0;

        for i in 1..arrays.len() {
            distance = distance.max((max - arrays[i][0]).abs()).max((arrays[i][arrays[i].len() - 1] - min).abs());

            max = max.max(arrays[i][arrays[i].len() - 1]);
            min = min.min(arrays[i][0]);
        }

        distance
    }
}

