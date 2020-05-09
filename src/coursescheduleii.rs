use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut stack : VecDeque<i32> = VecDeque::new();
        let mut map : HashMap<i32, Vec<i32>> = HashMap::new();
        let mut visited = vec![false; num_courses as usize];
        let mut path = vec![false; num_courses as usize];

        for i in prerequisites {
            if !map.contains_key(&i[1]) {
                map.insert(i[1], Vec::new());
            }

            (*map.get_mut(&i[1]).unwrap()).push(i[0]);
        }

        dbg!(&map);

        for i in 0..num_courses {
            if Self::go_through(&map, &mut stack, i, &mut visited, &mut path) {
                return vec![];
            }
        }

        stack.into_iter().collect::<Vec::<i32>>()
    }

    fn go_through(map: &HashMap<i32, Vec<i32>>, stack: &mut VecDeque<i32>, num: i32, visited: &mut Vec<bool>, path: &mut Vec<bool>) -> bool
    {
        if visited[num as usize] {
            return false;
        }

        if path[num as usize] {
            return true;
        }

        path[num as usize] = true;

        let mut result = false;
        if map.contains_key(&num) {
            for i in map.get(&num).unwrap() {
                if Self::go_through(map, stack, *i, visited, path) {
                    result = true;
                    break;
                }
            }
        }

        stack.push_front(num);

        path[num as usize] = false;
        visited[num as usize] = true;

        result
    }

    pub fn find_order_best(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::convert::{TryFrom, TryInto};
        std::thread::sleep(std::time::Duration::from_millis(2));
        Self::_find_order(
            num_courses as usize,
            prerequisites.iter().map(|item| {
                (
                    usize::try_from(item[0]).unwrap(),
                    usize::try_from(item[1]).unwrap(),
                )
            }),
        )
            .into_iter()
            .flatten()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()
            .unwrap()
    }

    /// idiomatic entry point.
    pub fn _find_order(n: usize, reqs: impl Iterator<Item = (usize, usize)>) -> Option<Vec<usize>> {
        // mapping of usize into it's requirements/depedency
        let reqs = reqs.fold(HashMap::new(), |mut acc, (item, req)| {
            acc.entry(item).or_insert(Vec::new()).push(req);
            acc
        });

        let mut res = Vec::new();
        let mut added = HashSet::<usize>::new();
        // for a depedency tree, no children may depend on it's
        // parent/grandparent/etc..
        let mut forbidden = HashSet::new();
        for i in 0..n {
            Self::add_depedency(i, &mut added, &reqs, &mut forbidden, &mut res)?;
        }
        Some(res)
    }

    /// Add element n as an depedency
    pub fn add_depedency<'r>(
        n: usize,
        added: &mut HashSet<usize>,
        reqs: &HashMap<usize, Vec<usize>>,
        forbidden: &mut HashSet<usize>,
        res: &'r mut Vec<usize>,
    ) -> Option<&'r mut Vec<usize>> {
        // early return if is a forbidden value
        if forbidden.contains(&n) {
            return None;
        }
        // early return if already inserted
        else if !added.insert(n) {
            return Some(res);
        };

        forbidden.insert(n);
        for i in reqs.get(&n).into_iter().flatten() {
            Self::add_depedency(*i, added, reqs, forbidden, res)?;
        }
        forbidden.remove(&n);

        res.push(n);
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_order_1() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    }

    #[test]
    fn find_order_2() {
        assert_eq!(Solution::find_order(4, vec![vec![1,0], vec![2,0], vec![3,1], vec![3,2]]), vec![0, 1, 2, 3]);
    }
}