struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a,b | a[0].cmp(&b[0]));
        let mut result : Vec<Vec<i32>> = vec![];
        for i in intervals {
            if result.len() == 0 || result.last().unwrap()[1] < i[0] {
                result.push(i.clone());   
            } else {
                result.last_mut().unwrap()[1] = i[1].max(result.last().unwrap()[1]);
            }
        }
        
        result
    }
}