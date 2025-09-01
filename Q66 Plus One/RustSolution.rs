pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut d = digits;
        let mut carry = 1; // add one

        for i in (0..d.len()).rev() {
            let total = d[i] + carry;
            d[i] = total % 10;
            carry = total / 10;
            if carry == 0 { break; }
        }

        if carry > 0 {
            d.insert(0, carry);
        }
        d
    }
}
