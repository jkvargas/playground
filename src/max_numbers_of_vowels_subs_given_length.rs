use std::cmp::max;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut window = Window::new(k);

        for i in &chars {
            window.add(i);
        }

        window.get_max()
    }
}

struct Window {
    vec: VecDeque<char>,
    current: i32,
    max: i32,
    k: usize,
}

impl Window {
    pub fn get_max(&self) -> i32 {
        self.max
    }

    pub fn new(k: i32) -> Window {
        Window {
            k: k as usize,
            vec: VecDeque::new(),
            current: 0,
            max: 0,
        }
    }

    pub fn add(&mut self, letter: &char) {
        if self.vec.len() < self.k {
            self.add_front(letter);
        } else {
            let letter_removed = self.vec.pop_back().unwrap();
            if is_vowel(&letter_removed) {
                self.current -= 1;
            }

            self.add_front(letter);
        }
    }

    fn add_front(&mut self, letter: &char) {
        self.vec.push_front(letter.clone());
        if is_vowel(letter) {
            self.current += 1;
            self.max = max(self.max, self.current);
        }
    }
}

fn is_vowel(letter: &char) -> bool {
    *letter == 'a'
        || *letter == 'A'
        || *letter == 'e'
        || *letter == 'E'
        || *letter == 'i'
        || *letter == 'I'
        || *letter == 'o'
        || *letter == 'O'
        || *letter == 'u'
        || *letter == 'U'
}
