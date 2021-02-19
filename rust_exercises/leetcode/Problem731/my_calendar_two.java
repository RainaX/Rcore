use std::collections::BTreeMap;

struct MyCalendarTwo {
    once: BTreeMap<i32, i32>,
    twice: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {

    fn new() -> Self {
        Self {
            once: BTreeMap::new(),
            twice: BTreeMap::new(),
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        let twice_left = self.twice.range(..=start).rev().next();
        let twice_right = self.twice.range(start..).next();
        
        if (!twice_left.is_none() && *twice_left.unwrap().1 > start) || (!twice_right.is_none() && *twice_right.unwrap().0 < end) {
            return false;
        }
        
        let once_left = match self.once.range(..=start).rev().next() {
            Some(x) => *x.0,
            None => start,
        };
        
        let temp: BTreeMap<i32, i32> = self.once.range(once_left..end)
            .map(|x| (*x.0, *x.1)).collect();
        
        let mut twice_parts = vec![];
        
        for (&s, &e) in temp.iter() {
            if e <= start {
                continue;
            }
            if s < start && e < end {
                twice_parts.push((start, e));
                self.twice.insert(start, e);
                self.once.insert(s, start);
            } else if e > end && s > start {
                twice_parts.push((s, end));
                self.twice.insert(s, end);
                self.once.insert(end, e);
                self.once.remove(&s);
            } else if s <= start && e >= end {
                twice_parts.push((start, end));
                self.twice.insert(start, end);
                if start > s {
                    self.once.insert(s, start);
                } else {
                    self.once.remove(&s);
                }
                
                if end < e {
                    self.once.insert(end, e);
                }
            } else {
                twice_parts.push((s, e));
                self.twice.insert(s, e);
                self.once.remove(&s);
            }
        }
        
        let mut time = start;
        for (s, e) in twice_parts {
            if time < s {
                self.once.insert(time, s);
            }
            time = e;
        }
        if time < end {
            self.once.insert(time, end);
        }
        
        return true;
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
