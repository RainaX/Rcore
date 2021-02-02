struct MaxStack {
    stack: Vec<i32>,
    max_stack: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MaxStack {
            stack: vec![],
            max_stack: vec![],
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.max_stack.len() == 0 {
            self.max_stack.push(x);
        } else {
            let max = self.max_stack[self.max_stack.len() - 1];
            self.max_stack.push(std::cmp::max(x, max));
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.max_stack.pop();
        self.stack.pop().unwrap()
    }
    
    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }
    
    fn peek_max(&self) -> i32 {
        self.max_stack[self.stack.len() - 1]
    }
    
    fn pop_max(&mut self) -> i32 {
        let max = self.max_stack[self.stack.len() - 1];
        let mut temp: Vec<i32> = vec![];
        loop {
            let x = self.pop();
            if x == max {
                break;
            }
            temp.push(x);
        }
        while temp.len() > 0 {
            self.push(temp.pop().unwrap());
        }
        max
    }
}

/**
 * Your MaxStack object will be instantiated and called as such:
 * let obj = MaxStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.peek_max();
 * let ret_5: i32 = obj.pop_max();
 */
