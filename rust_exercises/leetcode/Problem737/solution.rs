use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::collections::HashMap;

impl Solution {
    pub fn are_sentences_similar_two(words1: Vec<String>, words2: Vec<String>, pairs: Vec<Vec<String>>) -> bool {
        if words1.len() != words2.len() {
            return false;
        }
        
        let mut map = HashMap::new();
        for pair in pairs.into_iter() {
            let mut iter = pair.into_iter();
            let w1 = iter.next().unwrap();
            let w2 = iter.next().unwrap();
            
            if !map.contains_key(&w1) {
                map.insert(w1.clone(), Node::new(&w1));
            }
            if !map.contains_key(&w2) {
                map.insert(w2.clone(), Node::new(&w2));
            }
            
            Node::union(Rc::clone(map.get(&w1).unwrap()), Rc::clone(map.get(&w2).unwrap()));
        }
        
        let mut iter1 = words1.into_iter();
        let mut iter2 = words2.into_iter();
        
        loop {
            let w1 = match iter1.next() {
                Some(w) => w,
                None => break,
            };
            let w2 = iter2.next().unwrap();
            
            if w1 == w2 {
                continue;
            }
            
            if map.contains_key(&w1) && map.contains_key(&w2) && Node::similar(Rc::clone(map.get(&w1).unwrap()), Rc::clone(map.get(&w2).unwrap())) {
                continue;
            }
            
            return false;
        }
        
        true
    }
}


struct Node {
    word: String,
    parent: Weak<RefCell<Node>>,
    count: u32,
}

impl Node {
    fn new(word: &String) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            word: word.clone(),
            parent: Weak::new(),
            count: 1,
        }));
        node.borrow_mut().parent = Rc::downgrade(&node);
        node
    }
    
    fn get_root(node: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let mut curr = node;
        
        loop {
            let parent = curr.borrow().parent.upgrade().unwrap();
            if parent.borrow().word == curr.borrow().word {
                return curr;
            }
            curr = parent;
        }
    }
    
    fn union(n1: Rc<RefCell<Node>>, n2: Rc<RefCell<Node>>) {
        let root1 = Node::get_root(n1);
        let root2 = Node::get_root(n2);
        
        if root1.borrow().word == root2.borrow().word {
            return;
        }
        
        let count1 = root1.borrow().count;
        let count2 = root2.borrow().count;
        
        if count1 < count2 {
            root1.borrow_mut().parent = Rc::downgrade(&root2);
            root2.borrow_mut().count = count1 + count2;
        } else {
            root2.borrow_mut().parent = Rc::downgrade(&root1);
            root1.borrow_mut().count = count1 + count2;
        }
    }
    
    fn similar(n1: Rc<RefCell<Node>>, n2: Rc<RefCell<Node>>) -> bool {
        let root1 = Node::get_root(n1);
        let root2 = Node::get_root(n2);
        
        let ret = root1.borrow().word == root2.borrow().word;
        ret
    }
}
