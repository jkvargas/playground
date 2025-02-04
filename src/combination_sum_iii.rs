struct Solution;

impl Solution {
    pub fn combination_sum3(how_many: i32, aim: i32) -> Vec<Vec<i32>> {
        let mut results = Vec::new();

        bt(&mut results, &mut Vec::new(), aim, 1, how_many as usize);

        results
    }
}

fn bt(
    results: &mut Vec<Vec<i32>>,
    current: &mut Vec<i32>,
    remain: i32,
    start_from: i32,
    how_many: usize,
) {
    if current.len() == how_many && remain == 0 {
        results.push(current.clone());
        return;
    } else if remain < 0 || current.len() == how_many {
        return;
    }

    for i in start_from..10 {
        current.push(i);
        bt(results, current, remain - i, i + 1, how_many);
        current.pop();
    }
}

#[test]
fn test_one() {
    assert_eq!(vec![vec![1, 2, 4]], Solution::combination_sum3(3, 7));
}
