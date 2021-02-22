// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_closest_leaf(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let root = root.unwrap();
        map.insert(root.borrow().val, None);
        recordParent(Rc::clone(&root), &mut map);
        
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(k);
        visited.insert(k);
        
        
        let mut curr;
        loop {
            let val = queue.pop_front().unwrap();
            let parent = map.get(&val).unwrap();
            match parent.as_ref() {
                Some(p) => {
                    if p.borrow().left.is_some() && p.borrow().left.as_ref().unwrap().borrow().val == val {
                        curr = Rc::clone(p.borrow().left.as_ref().unwrap());
                    } else {
                        curr = Rc::clone(p.borrow().right.as_ref().unwrap());
                    }
                },
                None => {
                    curr = Rc::clone(&root);
                }
            }
            
            if curr.borrow().left.is_none() && curr.borrow().right.is_none() {
                return curr.borrow().val;
            }
            
            if parent.is_some() {
                let pval = parent.as_ref().unwrap().borrow().val;
                if !visited.contains(&pval) {
                    queue.push_back(pval);
                    visited.insert(pval);
                }
            }
            
            if let Some(lchild) = curr.borrow().left.as_ref() {
                let lval = lchild.borrow().val;
                if !visited.contains(&lval) {
                    queue.push_back(lval);
                    visited.insert(lval);
                }
            }
            
            if let Some(rchild) = curr.borrow().right.as_ref() {
                let rval = rchild.borrow().val;
                if !visited.contains(&rval) {
                    queue.push_back(rval);
                    visited.insert(rval);
                }
            }
        }
    }
}

fn recordParent(node: Rc<RefCell<TreeNode>>, map: &mut HashMap<i32, Option<Rc<RefCell<TreeNode>>>>) {
    if let Some(lchild) = node.borrow().left.as_ref() {
        map.insert(lchild.borrow().val, Some(Rc::clone(&node)));
        recordParent(Rc::clone(lchild), map);
    }
    
    if let Some(rchild) = node.borrow().right.as_ref() {
        map.insert(rchild.borrow().val, Some(Rc::clone(&node)));
        recordParent(Rc::clone(rchild), map);
    }
}
