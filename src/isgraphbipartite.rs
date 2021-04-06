use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut groupa: HashSet<i32> = HashSet::new();
        let mut groupb: HashSet<i32> = HashSet::new();
        let mut queue: Vec<i32> = Vec::new();
        let mut visited: Vec<bool> = vec![false; graph.len()];

        for pos in 0..graph.len() {
            if visited[pos] {
                continue;
            }

            queue.push(pos as i32);

            while !queue.is_empty() {
                let pop = queue.pop().unwrap();
                visited[pop as usize] = true;

                for i in &graph[pop as usize] {
                    if visited[*i as usize] {
                        continue;
                    }

                    if !Self::add_to_groups(pop, *i, &mut groupa, &mut groupb) {
                        return false;
                    }

                    queue.push(*i);
                }
            }
        }

        true
    }

    fn add_to_groups(pa: i32, pb: i32, a: &mut HashSet<i32>, b: &mut HashSet<i32>) -> bool {
        if !a.contains(&pa) && !b.contains(&pa) && !a.contains(&pb) && !b.contains(&pb) {
            a.insert(pa);
            b.insert(pb);
            return true;
        }

        if !a.contains(&pa) && b.contains(&pa) {
            if b.contains(&pb) {
                return false;
            } else {
                a.insert(pb);
            }
        } else if !b.contains(&pa) && a.contains(&pa) {
            if a.contains(&pb) {
                return false;
            } else {
                b.insert(pb);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //   GROUPA     GROUPB
    //     0          1
    //     0          3
    //     0          1
    //     2          1
    //     2          1

    #[test]
    fn is_bipartite_1() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]),
            true
        );
    }

    #[test]
    fn is_bipartite_2() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
            false
        );
    }

    #[test]
    fn is_bipartite_3() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![3], vec![2, 4], vec![1], vec![0, 4], vec![1, 3]]),
            true
        );
    }

    #[test]
    fn is_bipartite_4() {
        assert_eq!(
            Solution::is_bipartite(vec![
                vec![],
                vec![2, 4, 6],
                vec![1, 4, 8, 9],
                vec![7, 8],
                vec![1, 2, 8, 9],
                vec![6, 9],
                vec![1, 5, 7, 8, 9],
                vec![3, 6, 9],
                vec![2, 3, 4, 6, 9],
                vec![2, 4, 5, 6, 7, 8]
            ]),
            false
        );
    }

    #[test]
    fn is_bipartite_6() {
        assert_eq!(
            Solution::is_bipartite(vec![vec![], vec![3], vec![], vec![1], vec![]]),
            true
        );
    }
}
