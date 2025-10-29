// https://leetcode.com/problems/brace-expansion/?envType=study-plan-v2&envId=premium-algo-100

struct Solution;

impl Solution {
    pub fn expand(s: String) -> Vec<String> {
        let letters = s.chars().collect::<Vec<_>>();

        let mut v = Vec::new();
        let mut index = 0;

        while index < letters.len() {
            if letters[index] == '{' {
                let mut temp = Vec::new();
                while letters[index] != '}' {
                    if letters[index] != ',' && letters[index] != '{' {
                        temp.push(letters[index]);
                    }
                    index += 1;
                }
                v.push(temp);
            } else {
                if letters[index] != '}' {
                    v.push(vec![letters[index]]);
                }
                index += 1;
            }
        }

        let mut result = Vec::new();
        bt(&v, 0, &mut String::new(), &mut result);
        result.sort();
        result
    }
}

fn bt(v: &Vec<Vec<char>>, index: usize, word: &mut String, result: &mut Vec<String>) {
    if index == v.len() {
        result.push(word.clone());
        return;
    }

    for letter in &v[index] {
        word.push(*letter);
        bt(v, index + 1, word, result);
        word.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::brace_expansion::Solution;

    #[test]
    fn test_one() {
        Solution::expand("{a,b}{z,x,y}".into());
    }
}