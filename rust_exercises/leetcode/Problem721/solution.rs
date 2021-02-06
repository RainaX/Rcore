use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::HashMap;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut nodes = vec![];
        
        let mut map: HashMap<String, usize> = HashMap::new();
        
        for (id, account) in accounts.iter().enumerate() {
            nodes.push(Node::new(id));
            
            for email in account.iter().skip(1) {
                if (map.contains_key(email)) {
                    Node::union(Rc::clone(&nodes[*map.get(email).unwrap()]), Rc::clone(&nodes[id]));
                } else {
                    map.insert(email.clone(), id);
                }
            }
        }
        
        let mut result_map: HashMap<usize, Vec<String>> = HashMap::new();
        for (email, id) in map.into_iter() {
            let root_id = Node::get_root_id(Rc::clone(&nodes[id]));
            if !result_map.contains_key(&root_id) {
                result_map.insert(root_id, vec![]);
            }
            result_map.get_mut(&root_id).unwrap().push(email);
        }
        
        let mut result = vec![];
        for (id, emails) in result_map.iter_mut() {
            let mut account = vec![];
            account.push(accounts[*id][0].clone());
            emails.sort();
            account.extend_from_slice(emails);
            result.push(account);
        }
        
        result
    }
}

struct Node {
    id: usize,
    root: Weak<RefCell<Node>>,
    size: i32,
}

impl Node {
    fn new(id: usize) -> Rc<RefCell<Node>> {
        let node = Rc::new(RefCell::new(Node {
            id,
            root: Weak::new(),
            size: 1,
        }));
        node.borrow_mut().root = Rc::downgrade(&node);
        node
    }
    
    fn union(n1: Rc<RefCell<Node>>, n2: Rc<RefCell<Node>>) {
        let mut root1 = n1;
        loop {
            let r = root1.borrow().root.upgrade().unwrap();
            if r.borrow().id == root1.borrow().id {
                break;
            }
            root1 = r;
        }
        
        let mut root2 = n2;
        loop {
            let r = root2.borrow().root.upgrade().unwrap();
            if r.borrow().id == root2.borrow().id {
                break;
            }
            root2 = r;
        }
        
        if root1.borrow().id == root2.borrow().id {
            return;
        }
        
        if root1.borrow().size > root2.borrow().size {
            root2.borrow_mut().root = Rc::downgrade(&root1);
            let sum = root1.borrow().size + root2.borrow().size;
            root1.borrow_mut().size = sum;
        } else {
            root1.borrow_mut().root = Rc::downgrade(&root2);
            let sum = root1.borrow().size + root2.borrow().size;
            root2.borrow_mut().size = sum;
        }
    }
    
    fn get_root_id(node: Rc<RefCell<Node>>) -> usize {
        let mut root = node;
        loop {
            let r = root.borrow().root.upgrade().unwrap();
            if r.borrow().id == root.borrow().id {
                break;
            }
            root = r;
        }
        
        let ret = root.borrow().id;
        ret
    }
}
