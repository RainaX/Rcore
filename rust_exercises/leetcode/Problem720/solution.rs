use std::collections::HashMap;
use std::cmp::Ordering;

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut map: HashMap<String, i32> = HashMap::new();
        for word in words.iter() {
            if (word.len() == 1) {
                map.insert(word.clone(), 2);
            } else {
                map.insert(word.clone(), 0);
            }
        }
        
        let mut longest = String::new();
        
        for word in words.iter() {
            if is_valid(word, &mut map) {
                if word.len() > longest.len() {
                    longest = word.clone();
                } else if word.len() == longest.len() {
                    match word.cmp(&longest) {
                        Ordering::Less => longest = word.clone(),
                        _ => (),
                    }
                }
            }
        }
        
        longest
    }
}

fn is_valid(word: &str, map: &mut HashMap<String, i32>) -> bool {
    match map.get(word) {
        None | Some(1) => false,
        Some(2) => true,
        Some(_) => if is_valid(&word[..word.len() - 1], map) {
            map.insert(String::from(word), 2);
            true
        } else {
            map.insert(String::from(word), 1);
            false
        },
    }
}
