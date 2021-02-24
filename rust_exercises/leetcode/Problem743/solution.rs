use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::cmp::max;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut edges = HashMap::new();
        for edge in times.into_iter() {
            let src = edge[0];
            let dst = edge[1];
            let time = edge[2];
            if !edges.contains_key(&src) {
                edges.insert(src, vec![]);
            }
            edges.get_mut(&src).unwrap().push((dst, time));
        }
        
        let mut delay = HashMap::new();
        let mut pq = BinaryHeap::new();
        
        pq.push(Node {
            id: k,
            time: 0,
        });
        
        while !pq.is_empty() {
            let curr = pq.pop().unwrap();
            if delay.contains_key(&curr.id) {
                continue;
            }
            delay.insert(curr.id, curr.time);
            if !edges.contains_key(&curr.id) {
                continue;
            }
            
            for &(dst, time) in edges.get(&curr.id).unwrap().iter() {
                if !delay.contains_key(&dst) {
                    pq.push(Node {
                        id: dst,
                        time: curr.time + time,
                    });
                }
            }
        }
        
        if (delay.len() as i32) < n {
            return -1;
        }
        
        let mut result = 0;
        for (&_, &v) in delay.iter() {
            result = max(result, v);
        }
        result
    }
}

#[derive(Eq)]
struct Node {
    id: i32,
    time: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time != other.time {
            Reverse(self.time).cmp(&Reverse(other.time))
        } else {
            self.id.cmp(&other.time)
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.id == other.id
    }
}
