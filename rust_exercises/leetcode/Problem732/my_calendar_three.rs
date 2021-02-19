use std::collections::BTreeMap;

struct MyCalendarThree {
    map: BTreeMap<i32, (i32, i32)>,
    k: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {

    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            k: 0,
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> i32 {
        let prev = match self.map.range(..=start).rev().next() {
            Some(x) => *x.0,
            None => start,
        };
        
        let temp: BTreeMap<i32, (i32, i32)> = self.map.range(prev..end)
            .map(|x| (*x.0, ((x.1).0, (x.1).1))).collect();
        let mut covered = vec![];
        for (&s, &(e, count)) in temp.iter() {
            if e <= start {
                continue;
            }
            self.k = std::cmp::max(self.k, count + 1);
            if s < start && e < end {
                covered.push((start, e));
                self.map.insert(s, (start, count));
                self.map.insert(start, (e, count + 1));
            } else if e > end && s > start {
                covered.push((s, end));
                self.map.insert(s, (end, count + 1));
                self.map.insert(end, (e, count));
            } else if s <= start && e >= end {
                covered.push((start, end));
                if start > s {
                    self.map.insert(s, (start, count));
                } else {
                    self.map.remove(&s);
                }
                
                if end < e {
                    self.map.insert(end, (e, count));
                }
                self.map.insert(start, (end, count + 1));
            } else {
                covered.push((s, e));
                self.map.insert(s, (e, count + 1));
            }
        }
        
        let mut time = start;
        for &(s, e) in covered.iter() {
            if time < s {
                self.map.insert(time, (s, 1));
            }
            time = e;
        }
        if time < end {
            self.map.insert(time, (end, 1));
        }
        self.k = std::cmp::max(self.k, 1);
        
        self.k
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(start, end);
 */
