use std::collections::{HashMap, HashSet};

struct SolutionJho;

struct Solution;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Position {
    i: i32,
    j: i32,
}

impl Position {
    pub fn get_paths(
        &self,
        m: i32,
        n: i32,
        out_of_bondary: &mut HashMap<Position, HashSet<Position>>,
        paths: &mut HashMap<Position, HashSet<Position>>,
    ) {
        if out_of_bondary.contains_key(self) {
            return;
        }

        self.feed(
            Position {
                i: self.i + 1,
                j: self.j,
            },
            m,
            n,
            out_of_bondary,
            paths,
        );

        self.feed(
            Position {
                i: self.i - 1,
                j: self.j,
            },
            m,
            n,
            out_of_bondary,
            paths,
        );

        self.feed(
            Position {
                i: self.i,
                j: self.j + 1,
            },
            m,
            n,
            out_of_bondary,
            paths,
        );

        self.feed(
            Position {
                i: self.i,
                j: self.j - 1,
            },
            m,
            n,
            out_of_bondary,
            paths,
        );
    }

    fn feed(
        &self,
        pos: Position,
        m: i32,
        n: i32,
        out_of_bondary: &mut HashMap<Position, HashSet<Position>>,
        paths: &mut HashMap<Position, HashSet<Position>>,
    ) {
        if pos.i > m - 1 || pos.i < 0 || pos.j > n - 1 || pos.j < 0 {
            if out_of_bondary.contains_key(self) {
                out_of_bondary.get_mut(self).unwrap().insert(pos);
            } else {
                let mut hs = HashSet::new();
                hs.insert(pos);
                out_of_bondary.insert(self.clone(), hs);
            }
        } else {
            if paths.contains_key(self) {
                paths.get_mut(self).unwrap().insert(pos);
            } else {
                let mut hs = HashSet::new();
                hs.insert(pos);
                paths.insert(self.clone(), hs);
            }
        }
    }
}

impl SolutionJho {
    pub fn find_paths(m: i32, n: i32, n_mov: i32, i: i32, j: i32) -> i32 {
        let mut out = HashMap::new();
        let mut path = HashMap::new();
        let mut results = Vec::new();

        let position = Position { i, j };

        Self::f(m, n, n_mov, &position, &mut out, &mut path, &mut results);

        results.len() as i32
    }

    fn f(
        m: i32,
        n: i32,
        n_mov: i32,
        pos: &Position,
        out: &mut HashMap<Position, HashSet<Position>>,
        path: &mut HashMap<Position, HashSet<Position>>,
        set: &mut Vec<Position>,
    ) {
        if n_mov == 0 {
            return;
        }

        pos.get_paths(m, n, out, path);

        let positions = path.get(&pos).unwrap().clone();

        for i in positions {
            Self::f(m, n, n_mov - 1, &i, out, path, set);
        }

        for i in out.get(&pos).unwrap() {
            set.push(i.clone());
        }
    }
}

const M: i32 = 1000000000 + 7;

impl Solution {
    pub fn find_paths(m: i32, n: i32, n_mov: i32, i: i32, j: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];
        dp[i as usize][j as usize] = 1;
        let mut count = 0;

        for _moves in 1..=n_mov {
            let mut temp = vec![vec![0; n as usize]; m as usize];
            for x in 0..m as usize {
                for y in 0..n as usize {
                    if x == m as usize - 1 {
                        count = (count + dp[x][y]) % M;
                    }
                    if y == n as usize - 1 {
                        count = (count + dp[x][y]) % M;
                    }
                    if x == 0 {
                        count = (count + dp[x][y]) % M;
                    }
                    if y == 0 {
                        count = (count + dp[x][y]) % M;
                    }

                    let before_x = if x > 0 { dp[x - 1][y] } else { 0 };
                    let after_x = if x < m as usize - 1 { dp[x + 1][y] } else { 0 };
                    let before_y = if y > 0 { dp[x][y - 1] } else { 0 };
                    let after_y = if y < n as usize - 1 { dp[x][y + 1] } else { 0 };

                    temp[x][y] = (((before_x + after_x) % M) + ((before_y + after_y) % M)) % M;
                }
            }

            dp = temp;
        }

        count
    }
}

#[test]
fn test_one() {
    assert_eq!(6, Solution::find_paths(2, 2, 2, 0, 0));
}

#[test]
fn test_sec() {
    assert_eq!(12, Solution::find_paths(1, 3, 3, 0, 1));
}
