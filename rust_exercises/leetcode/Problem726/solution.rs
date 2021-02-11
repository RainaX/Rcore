use std::collections::HashMap;
use std::collections::BTreeMap;
use std::cmp::max;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut formula: Vec<char> = formula.as_bytes().iter().map(|x| *x as char).collect();
        
        let mut open2time: HashMap<u32, u32> = HashMap::new();
        let mut close2skip: HashMap<u32, u32> = HashMap::new();
        
        let mut i = 0;
        let mut temp: Vec<u32> = vec![];
        
        while i < formula.len() {
            let c = formula[i];
            
            i += 1;
            
            if c == '(' {
                temp.push(i as u32 - 1);
            } else if c == ')' {
                let mut time = 0;
                let j = i - 1;
                while i < formula.len() && formula[i].is_digit(10) {
                    time = time * 10 + formula[i].to_digit(10).unwrap();
                    i += 1;
                }
                time = max(1, time);
                open2time.insert(temp.pop().unwrap() as u32, time);
                close2skip.insert(j as u32, i as u32);
            }
        }
        
        let mut map = BTreeMap::new();
        let mut count = 0;
        let mut time = 1;
        let mut element = String::new();
        
        i = 0;
        
        while i < formula.len() {
            let c = formula[i];
            
            if c.is_uppercase() || c == '(' || c == ')' {
                if element.len() > 0 {
                    count = max(count, 1);
                    match map.get(&element) {
                        Some(x) => map.insert(element, x + count * time),
                        None => map.insert(element, count * time),
                    };
                    element = String::new();
                    count = 0;
                }
            }
            
            if c == ')' {
                time /= temp.pop().unwrap();
                i = *close2skip.get(&(i as u32)).unwrap() as usize;
            } else {
                if c == '(' {
                    temp.push(*open2time.get(&(i as u32)).unwrap());
                    time *= temp[temp.len() - 1];
                } else if c.is_alphabetic() {
                    element.push(c);
                } else if c.is_digit(10) {
                    count = count * 10 + c.to_digit(10).unwrap();
                }
                i += 1;
            }
        }
        
        if element.len() > 0 {
            count = max(count, 1);
            match map.get(&element) {
                Some(x) => map.insert(element, x + count * time),
                None => map.insert(element, count * time),
            };
        }
        
        let mut result = String::new();
        
        for (element, count) in map.iter() {
            result.push_str(&element);
            if *count > 1 {
                result.push_str(&count.to_string());
            }
        }
        
        result
    }
}
