use std::collections::VecDeque;

struct Solution;

enum Action {
    Return,
    Go(String),
    None,
}

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut directories = Vec::new();
        let mut letters: VecDeque<char> = path.chars().collect();
        let mut word = Self::consume_word(&mut letters);

        loop {
            match word {
                Action::Return => {
                    directories.pop();
                }
                Action::Go(dir) => {
                    directories.push(dir);
                }
                Action::None => {
                    if letters.is_empty() {
                        break;
                    }
                }
            }

            word = Self::consume_word(&mut letters);
        }
        format!("/{}", directories.join("/"))
    }

    fn consume_word(path: &mut VecDeque<char>) -> Action {
        let mut dir = Vec::new();

        while !path.is_empty() {
            let letter = path.pop_front().unwrap();

            if letter != '/' {
                dir.push(letter);
            } else if !dir.is_empty() {
                return Self::consume_from_dir(&dir);
            }
        }

        if !dir.is_empty() {
            Self::consume_from_dir(&dir)
        } else {
            Action::None
        }
    }

    fn consume_from_dir(dir: &Vec<char>) -> Action {
        let dots = Self::count_dots(&dir);
        if dots > 0 {
            if dots == 1 {
                return Action::None;
            } else if dots == 2 {
                return Action::Return;
            }
        }

        return Action::Go(dir.into_iter().collect());
    }

    fn count_dots(vec: &Vec<char>) -> i32 {
        let mut dots = 0;
        for i in vec {
            if *i == '.' {
                dots += 1;
            } else {
                return 0;
            }
        }

        dots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplify_path_1() {
        let word = Solution::simplify_path("/home/".to_string());
        assert_eq!(word, "/home".to_string());
    }

    #[test]
    fn simplify_path_2() {
        assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
    }

    #[test]
    fn simplify_path_3() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
    }

    #[test]
    fn simplify_path_7() {
        assert_eq!(
            Solution::simplify_path("/...".to_string()),
            "/...".to_string()
        );
    }

    #[test]
    fn simplify_path_4() {
        assert_eq!(
            Solution::simplify_path("/a/./b/../../c/".to_string()),
            "/c".to_string()
        );
    }

    #[test]
    fn simplify_path_5() {
        assert_eq!(
            Solution::simplify_path("/a/../../b/../c//.//".to_string()),
            "/c".to_string()
        );
    }

    #[test]
    fn simplify_path_6() {
        assert_eq!(
            Solution::simplify_path("/a//b////c/d//././/..".to_string()),
            "/a/b/c".to_string()
        );
    }
}
