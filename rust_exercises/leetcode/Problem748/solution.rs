impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut license_map = vec![0; 26];
        for ch in license_plate.chars() {
            if ch.is_alphabetic() {
                let index = ch.to_lowercase().next().unwrap() as usize - 'a' as usize;
                license_map[index] = license_map[index] + 1;
            }
        }
        
        let mut shortest: Option<&String> = None;
        
        for word in words.iter() {
            let mut word_map = vec![0; 26];
            for ch in word.chars() {
                let index = ch as usize - 'a' as usize;
                word_map[index] = word_map[index] + 1;
            }
            
            let mut i = 0;
            while i < 26 {
                if word_map[i] < license_map[i] {
                    break;
                }
                i += 1;
            }
            if (i >= 26 && (shortest.is_none() || word.len() < shortest.unwrap().len())) {
                shortest = Some(word);
            }
        }
        
        shortest.unwrap().clone()
    }
}
