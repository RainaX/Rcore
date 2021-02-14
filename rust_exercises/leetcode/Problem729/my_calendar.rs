use std::collections::BTreeMap;

struct MyCalendar {
    map: BTreeMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {

    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        let prev = self.map.range(..=start).rev().next();
        let next = self.map.range(start..).next();
        
        if (prev.is_none() || *prev.unwrap().1 <= start) && (next.is_none() || *next.unwrap().0 >= end) {
            self.map.insert(start, end);
            return true;
        }
        false
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
