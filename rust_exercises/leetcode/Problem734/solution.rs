use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;

impl Solution {
    pub fn are_sentences_similar(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool {
        if sentence1.len() != sentence2.len() {
            return false;
        }
        
        let mut map = HashMap::new();
        for pair in similar_pairs.into_iter() {
            let mut iter = pair.into_iter();
            let w1 = iter.next().unwrap();
            let w2 = iter.next().unwrap();
            
            if !map.contains_key(&w1) {
                map.insert(w1.clone(), RefCell::new(HashSet::new()));
            }
            map.get(&w1).unwrap().borrow_mut().insert(w2);
        }
        
        for i in 0..sentence1.len() {
            let w1 = &sentence1[i];
            let w2 = &sentence2[i];
            
            if *w1 != *w2 && !(map.contains_key(w1) && map.get(w1).unwrap().borrow().contains(w2)) && !(map.contains_key(w2) && map.get(w2).unwrap().borrow().contains(w1)) {
                return false;
            }
        }
        
        true
    }
}
