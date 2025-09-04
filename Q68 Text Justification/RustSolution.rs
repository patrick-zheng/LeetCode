pub struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let n = words.len();
        let maxw = max_width as usize;
        let mut i: usize = 0;

        while i < n {
            let mut j = i;
            let mut line_len: usize = 0;

            // Greedily take as many words as fit: letters + gaps (j - i)
            while j < n {
                let needed = line_len + words[j].len() + (j - i);
                if needed > maxw {
                    break;
                }
                line_len += words[j].len();
                j += 1;
            }

            let gaps = j.saturating_sub(i + 1);
            let last_line = j == n;
            let mut line = String::new();

            if gaps == 0 || last_line {
                // Left-justify
                line.push_str(&words[i]);
                for k in (i + 1)..j {
                    line.push(' ');
                    line.push_str(&words[k]);
                }
                let pad = maxw - line.len();
                line.extend(std::iter::repeat(' ').take(pad));
            } else {
                // Fully justify
                let total_spaces = maxw - line_len;
                let base = total_spaces / gaps;
                let mut extra = total_spaces % gaps;

                for k in i..(j - 1) {
                    line.push_str(&words[k]);
                    let mut spaces = base;
                    if extra > 0 {
                        spaces += 1;
                        extra -= 1;
                    }
                    line.extend(std::iter::repeat(' ').take(spaces));
                }
                line.push_str(&words[j - 1]);
            }

            res.push(line);
            i = j;
        }

        res
    }
}
