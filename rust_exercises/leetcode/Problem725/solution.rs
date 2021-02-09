// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn split_list_to_parts(root: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut len: i32 = 0;
        let mut curr = &root;
        while !curr.is_none() {
            len += 1;
            curr = &curr.as_ref().unwrap().next;
        }
        
        let average = len / k;
        let extra = len % k;
        let mut result = vec![];
        
        let mut curr = root;
        for i in 0..k {
            let mut temp = vec![];
            for j in 0..(average + (i < extra) as i32) {
                temp.push(curr.as_ref().unwrap().val);
                curr = curr.unwrap().next;
            }
            let mut next = None;
            loop {
                let val = match temp.pop() {
                    Some(x) => x,
                    None => break,
                };
                let mut node = ListNode::new(val);
                node.next = next;
                next = Some(Box::new(node));
            }
            result.push(next);
        }
        
        result
    }
}
