pub struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        const BELOW_20: [&str; 20] = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight",
            "Nine", "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen",
            "Sixteen", "Seventeen", "Eighteen", "Nineteen",
        ];
        const TENS: [&str; 10] = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy",
            "Eighty", "Ninety",
        ];
        const UNITS: [&str; 4] = ["", "Thousand", "Million", "Billion"];

        fn chunk_to_words(n: i32) -> String {
            if n == 0 {
                return String::new();
            }
            if n < 20 {
                return BELOW_20[n as usize].to_string();
            }
            if n < 100 {
                let mut s = TENS[(n / 10) as usize].to_string();
                if n % 10 != 0 {
                    s.push(' ');
                    s.push_str(BELOW_20[(n % 10) as usize]);
                }
                return s;
            }
            let mut s = format!("{} Hundred", BELOW_20[(n / 100) as usize]);
            if n % 100 != 0 {
                s.push(' ');
                s.push_str(&chunk_to_words(n % 100));
            }
            s
        }

        let mut parts: Vec<String> = Vec::new();
        let mut value = num;
        for i in 0..4 {
            let chunk = value % 1000;
            if chunk != 0 {
                let mut part = chunk_to_words(chunk);
                if !UNITS[i].is_empty() {
                    part.push(' ');
                    part.push_str(UNITS[i]);
                }
                parts.push(part);
            }
            value /= 1000;
        }

        parts.into_iter().rev().collect::<Vec<_>>().join(" ")
    }
}
