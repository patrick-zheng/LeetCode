pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let roman_map: HashMap<char, i32> = [
            ('I', 1), ('V', 5), ('X', 10), ('L', 50),
            ('C', 100), ('D', 500), ('M', 1000),
        ].iter().cloned().collect();

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;

        for i in 0..chars.len() {
            let curr = roman_map[&chars[i]];
            let next = roman_map.get(&chars.get(i + 1).cloned().unwrap_or(' ')).cloned().unwrap_or(0);

            if curr < next { total -= curr; }
            else { total += curr; }
        }

        total
    }
}

