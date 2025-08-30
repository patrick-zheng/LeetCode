struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        fn is_digits(t: &str) -> bool {
            !t.is_empty() && t.chars().all(|c| c.is_ascii_digit())
        }

        fn is_int(mut t: &str, allow_sign: bool) -> bool {
            if allow_sign {
                if let Some(rest) = t.strip_prefix('+') { t = rest; }
                else if let Some(rest) = t.strip_prefix('-') { t = rest; }
            }
            is_digits(t)
        }

        fn is_dec(mut t: &str, allow_sign: bool) -> bool {
            if allow_sign {
                if let Some(rest) = t.strip_prefix('+') { t = rest; }
                else if let Some(rest) = t.strip_prefix('-') { t = rest; }
            }
            // exactly one dot
            if t.matches('.').count() != 1 { return false; }
            let (left, right) = t.split_once('.').unwrap();
            // at least one side has digits; both sides (if non-empty) must be digits
            if left.is_empty() && right.is_empty() { return false; }
            if !left.is_empty() && !left.chars().all(|c| c.is_ascii_digit()) { return false; }
            if !right.is_empty() && !right.chars().all(|c| c.is_ascii_digit()) { return false; }
            true
        }

        let s = s.trim();
        if s.is_empty() { return false; }

        // find a single 'e' or 'E'
        let mut e_pos: Option<usize> = None;
        for (i, c) in s.chars().enumerate() {
            if c == 'e' || c == 'E' {
                if e_pos.is_some() { return false; }
                e_pos = Some(i);
            }
        }

        if let Some(pos) = e_pos {
            let base = &s[..pos];
            let exp  = &s[pos + 1..];
            if base.is_empty() || exp.is_empty() { return false; }
            let base_ok = is_int(base, true) || is_dec(base, true);
            let exp_ok  = is_int(exp, true); // exponent must be an integer
            base_ok && exp_ok
        } else {
            is_int(s, true) || is_dec(s, true)
        }
    }
}
