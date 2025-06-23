pub struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = num_rows as usize;
        let length = s.len();

        if (num_rows == 1 || num_rows >= length) { return s; }

        let mut rows: Vec<String> = vec![String::new(); num_rows];
        let mut curr_row: usize = 0;
        let mut going_down: bool = false;

        for c in s.chars() {
            rows[curr_row].push(c);
            if (curr_row == 0 || curr_row == num_rows - 1) {
                going_down = !going_down;
            }
            curr_row = if going_down { curr_row + 1 } else { curr_row - 1 };
        }
        rows.concat()
    }
}

