use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub type Clip = Vec<i32>;

struct Solution;

pub struct Movie {
    time: i32,
    parts: HashSet<i32>,
    clips_added: BTreeSet<Clip>,
    clips_removed: HashSet<Clip>,
}

impl Movie {
    pub fn new(time: i32) -> Movie {
        Movie {
            time: time + 1, // including 0
            parts: HashSet::new(),
            clips_added: BTreeSet::new(),
            clips_removed: HashSet::new(),
        }
    }

    #[inline]
    pub fn is_complete(&mut self) -> bool {
        self.parts.len() as i32 == self.time
    }

    #[inline]
    pub fn get_parts_len(&self) -> usize {
        self.clips_added.len()
    }

    pub fn add_clip_if_possible(&mut self, clip: &Clip) -> bool {
        if !self.clips_added.contains(clip) && !self.clips_removed.contains(clip) {
            for i in clip[0]..=clip[1] {
                self.parts.insert(i);
            }

            self.clips_added.insert(clip.clone());
            return true;
        }

        false
    }

    #[inline]
    pub fn stop_ignoring(&mut self, clip: &Clip) {
        self.clips_removed.remove(clip);
    }

    pub fn remove_and_ignore(&mut self, clip: &Clip) {
        if self.clips_added.contains(clip) {
            for i in clip[0]..=clip[1] {
                self.parts.remove(&i);
            }

            self.clips_removed.insert(clip.clone());
            self.clips_added.remove(clip);
        }
    }
}

impl Solution {
    pub fn video_stitching(mut clips: Vec<Clip>, time: i32) -> i32 {
        let mut movie = Movie::new(time);
        let mut clips_hash = HashSet::new();
        let mut memo = HashMap::new();

        while !clips.is_empty() {
            let pop = clips.pop().unwrap();
            clips_hash.insert(pop);
        }

        Self::f(&clips_hash, &mut movie, &mut memo)
    }

    fn f(clips: &HashSet<Clip>, movie: &mut Movie, memo: &mut HashMap<BTreeSet<Clip>, i32>) -> i32 {
        if let Some(&m) = memo.get(&movie.clips_added) {
            return m;
        }

        if movie.is_complete() {
            return movie.get_parts_len() as i32;
        }

        for i in clips {
            if !movie.add_clip_if_possible(i) {
                continue;
            }

            let with_number = Self::f(clips, movie, memo);

            movie.remove_and_ignore(i);

            let without_number = Self::f(clips, movie, memo);

            movie.stop_ignoring(i);

            let res = Self::best_result(with_number, without_number);

            memo.insert(movie.clips_added.clone(), res);

            return res;
        }

        memo.insert(movie.clips_added.clone(), -1);
        -1
    }

    fn best_result(with_number: i32, without_number: i32) -> i32 {
        if with_number != -1 {
            if without_number != -1 {
                return min(with_number, without_number);
            } else {
                return with_number;
            }
        }

        if without_number != -1 {
            return without_number;
        }

        -1
    }
}

#[test]
fn test_one() {
    assert_eq!(
        3,
        Solution::video_stitching(
            vec![
                vec![0, 2],
                vec![4, 6],
                vec![8, 10],
                vec![1, 9],
                vec![1, 5],
                vec![5, 9]
            ],
            10,
        )
    );
}

#[test]
fn test_two() {
    assert_eq!(
        -1,
        Solution::video_stitching(vec![vec![0, 1], vec![1, 2]], 5)
    );
}

#[test]
fn test_three() {
    assert_eq!(
        3,
        Solution::video_stitching(
            vec![
                vec![0, 1],
                vec![6, 8],
                vec![0, 2],
                vec![5, 6],
                vec![0, 4],
                vec![0, 3],
                vec![6, 7],
                vec![1, 3],
                vec![4, 7],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
                vec![3, 4],
                vec![4, 5],
                vec![5, 7],
                vec![6, 9]
            ],
            9,
        )
    );
}

#[test]
fn test_four() {
    assert_eq!(
        2,
        Solution::video_stitching(vec![vec![0, 4], vec![2, 8]], 5,)
    );
}
