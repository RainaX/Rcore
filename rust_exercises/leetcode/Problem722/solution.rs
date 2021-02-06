impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut block = false;
        let mut i: usize = 0;
        let mut pos: usize = 0;
        let mut temp = String::new();
        
        while i < source.len() {
            if block {
                let pos_end = source[i][pos..].find("*/");
                if pos_end.is_none() {
                    i += 1;
                    pos = 0;
                } else {
                    pos += pos_end.unwrap() + 2;
                    block = false;
                    if pos >= source[i].len() {
                        i += 1;
                        pos = 0;
                        if temp.len() > 0 {
                            result.push(temp);
                            temp = String::new();
                        }
                    }
                }
            } else {
                let remain = &source[i][pos..];
                let pos_line = match remain.find("//") {
                    Some(x) => x,
                    None => remain.len(),
                };
                let pos_block = match remain.find("/*") {
                    Some(x) => x,
                    None => remain.len(),
                };
                
                if pos_line == pos_block {
                    temp.push_str(remain);
                    i += 1;
                    pos = 0;
                    result.push(temp);
                    temp = String::new();
                } else if pos_line < pos_block {
                    temp.push_str(&remain[..pos_line]);
                    i += 1;
                    pos = 0;
                    if temp.len() > 0 {
                        result.push(temp);
                        temp = String::new();
                    }
                } else {
                    temp.push_str(&remain[..pos_block]);
                    pos += pos_block + 2;
                    block = true;
                    if pos >= source[i].len() {
                        i += 1;
                        pos = 0;
                    }
                }
            }
        }
        
        result
    }
}
