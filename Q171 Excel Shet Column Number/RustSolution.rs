pub struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result: i64 = 0;

        for b in column_title.bytes() {
            let val = (b - b'A' + 1) as i64; // A->1 ... Z->26
            result = result * 26 + val;
        }

        result as i32
    }
}
