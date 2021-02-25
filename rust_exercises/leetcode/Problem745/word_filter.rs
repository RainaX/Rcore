use std::collections::HashMap;

struct WordFilter {
    suffix_root: SuffixNode,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordFilter {

    fn new(words: Vec<String>) -> Self {
        let mut filter = Self {
            suffix_root: SuffixNode::new(),
        };
        
        for (i, word) in words.iter().enumerate() {
            filter.suffix_root.insert(word, i as i32);
        }
        
        filter
    }
    
    fn f(&self, prefix: String, suffix: String) -> i32 {
        self.suffix_root.search(&prefix, &suffix)
    }
}

struct SuffixNode {
    children: HashMap<char, SuffixNode>,
    prefix_root: PrefixNode,
}

impl SuffixNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            prefix_root: PrefixNode::new(),
        }
    }
    
    fn insert(&mut self, word: &String, index: i32) {
        let mut curr = self;
        for ch in word.chars().rev() {
            if !curr.children.contains_key(&ch) {
                curr.children.insert(ch, SuffixNode::new());
            }
            curr = curr.children.get_mut(&ch).unwrap();
            curr.prefix_root.insert(word, index);
        }
    }
    
    fn search(&self, prefix: &String, suffix: &String) -> i32 {
        let mut curr = self;
        for ch in suffix.chars().rev() {
            if !curr.children.contains_key(&ch) {
                return -1;
            }
            curr = curr.children.get(&ch).unwrap();
        }
        
        curr.prefix_root.search(prefix)
    }
}

struct PrefixNode {
    children: HashMap<char, PrefixNode>,
    max_index: i32,
}

impl PrefixNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            max_index: -1,
        }
    }
    
    fn insert(&mut self, word: &String, index: i32) {
        let mut curr = self;
        for ch in word.chars() {
            if !curr.children.contains_key(&ch) {
                curr.children.insert(ch, PrefixNode::new());
            }
            curr = curr.children.get_mut(&ch).unwrap();
            curr.max_index = std::cmp::max(curr.max_index, index);
        }
    }
    
    fn search(&self, prefix: &String) -> i32 {
        let mut curr = self;
        for ch in prefix.chars() {
            if !curr.children.contains_key(&ch) {
                return -1;
            }
            curr = curr.children.get(&ch).unwrap();
        }
        curr.max_index
    }
}

/**
 * Your WordFilter object will be instantiated and called as such:
 * let obj = WordFilter::new(words);
 * let ret_1: i32 = obj.f(prefix, suffix);
 */
