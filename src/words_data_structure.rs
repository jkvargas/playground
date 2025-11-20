use std::collections::HashMap;

#[derive(Default, Debug)]
struct WordDictionary {
    root: TrieNode,
}

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    end_of_word: bool,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        Self {
            root: Default::default(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut current = &mut self.root;
        for i in word.chars() {
            current = current.children.entry(i).or_default();
        }
        current.end_of_word = true;
    }

    fn search(&self, word: String) -> bool {
        let letters = word.chars().collect::<Vec<_>>();
        let mut current = vec![&self.root];
        let mut letter_index = 0;
        while letter_index < letters.len() {
            if letter_index >= letters.len() {
                return true;
            }
            let letter = letters[letter_index];
            if letter == '.' {
                let mut new_vec = vec![];
                for &child in &current {
                    for item in child.children.values() {
                        new_vec.push(item);
                    }
                }
                current = new_vec;
                letter_index += 1;
                continue;
            }

            if letter == letters[letter_index] {
                let mut new_vec = vec![];
                let mut not_found = true;
                for &child in &current {
                    if child.children.contains_key(&letters[letter_index]) {
                        new_vec.push(&child.children[&letters[letter_index]]);
                        not_found = false;
                    }
                }
                if not_found {
                    return false;
                }

                current = new_vec;
                letter_index += 1;
            }
        }

        current.iter().any(|x| x.end_of_word)
    }
}

#[cfg(test)]
mod tests {
    use crate::words_data_structure::WordDictionary;

    #[test]
    fn test_one() {
        let mut wd = WordDictionary::new();
        wd.add_word("altered beast".into());
        assert!(!wd.search("altered".into()));
        assert!(!wd.search("alt..ed".into()));
        assert!(wd.search("altered bea..".into()));
    }
}
