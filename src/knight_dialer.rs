use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let div = 1_000_000_007;
        let mut jump_map = HashMap::new();
        jump_map.insert(0, vec![4, 6]);
        jump_map.insert(1, vec![6, 8]);
        jump_map.insert(2, vec![7, 9]);
        jump_map.insert(3, vec![4, 8]);
        jump_map.insert(4, vec![3, 9, 0]);
        jump_map.insert(5, vec![]);
        jump_map.insert(6, vec![7, 1, 0]);
        jump_map.insert(7, vec![2, 6]);
        jump_map.insert(8, vec![1, 3]);
        jump_map.insert(9, vec![2, 4]);

        let mut memo = HashMap::new();
        let mut res = 0;

        for i in 0..10 {
            res = (res + run_jump(&jump_map, &mut memo, n - 1, i)) % div;
        }

        res
    }
}

fn run_jump(jump_map: &HashMap<i32, Vec<i32>>, memo: &mut HashMap<(i32, i32), i32>, n: i32, starting: i32) -> i32 {
    if n == 0 { return 1; }

    if memo.contains_key(&(starting, n)) {
        return *memo.get(&(starting, n)).unwrap();
    }

    let div = 1_000_000_007;
    let mut jump = 0;
    let from_start = jump_map.get(&starting).unwrap();
    for dest in from_start {
        jump = (jump + run_jump(jump_map, memo, n - 1, *dest)) % div;
    }

    memo.insert((starting, n), jump);
    jump
}