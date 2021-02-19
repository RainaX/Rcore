use std::collections::HashMap;
use std::cell::RefCell;

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let mut paren = HashMap::new();
        let mut temp = vec![];
        
        for (i, c) in expression.chars().enumerate() {
            if c == '(' {
                temp.push(i);
            } else if c == ')' {
                paren.insert(temp.pop().unwrap(), i);
            }
        }
        
        let mut values = HashMap::new();
        
        eval(&expression, 0, expression.len() - 1, &mut values, &paren)
    }
}

fn eval(expr: &String, l: usize, r: usize, values: &mut HashMap<String, RefCell<Vec<i32>>>, paren: &HashMap<usize, usize>) -> i32 {
    if &expr[l..(l + 1)] == "(" {
        return eval(expr, l + 1, r - 1, values, paren);
    }
    
    if r - l >= 3 && &expr[l..(l + 4)] == "let " {
        let mut new_assigned = vec![];
        let mut pos = l + 4;
        
        loop {
            let vr = get_right(expr, pos, r, paren);
            if vr == r {
                break;
            }
            let v = String::from(&expr[pos..(vr + 1)]);
            pos = vr + 2;
            
            let er = get_right(expr, pos, r, paren);
            let e = eval(expr, pos, er, values, paren);
            pos = er + 2;
            
            new_assigned.push(v.clone());
            if !values.contains_key(&v) {
                values.insert(v.clone(), RefCell::new(vec![]));
            }
            values.get(&v).unwrap().borrow_mut().push(e);
        }
        
        let ret = eval(expr, pos, r, values, paren);
        while new_assigned.len() > 0 {
            let v = new_assigned.pop().unwrap();
            values.get(&v).unwrap().borrow_mut().pop();
            if values.get(&v).unwrap().borrow().len() == 0 {
                values.remove(&v);
            }
        }
        return ret;
    }
    
    if r - l >= 3 && &expr[l..(l + 4)] == "add " {
        let mut pos = l + 4;
        
        let e1r = get_right(expr, pos, r, paren);
        let e1 = eval(expr, pos, e1r, values, paren);
        pos = e1r + 2;
        
        let e2 = eval(expr, pos, r, values, paren);
        
        return e1 + e2;
    }
    
    if r - l >= 4 && &expr[l..(l + 5)] == "mult " {
        let mut pos = l + 5;
        
        let e1r = get_right(expr, pos, r, paren);
        let e1 = eval(expr, pos, e1r, values, paren);
        pos = e1r + 2;
        
        let e2 = eval(expr, pos, r, values, paren);
        
        return e1 * e2;
    }
    
    let c = expr.as_bytes()[l] as char;
    if (c.is_lowercase()) {
        let temp = values.get(&expr[l..(r + 1)]).unwrap().borrow();
        return temp[temp.len() - 1];
    }
    
    String::from(&expr[l..(r + 1)]).parse().unwrap()
}


fn get_right(expr: &String, l: usize, r: usize, paren: &HashMap<usize, usize>) -> usize {
    if &expr[l..(l + 1)] == "(" {
        return *paren.get(&l).unwrap();
    }
    
    let mut right = l;
    while right <= r && &expr[right..(right + 1)] != " " {
        right += 1;
    }
    
    right - 1
}
