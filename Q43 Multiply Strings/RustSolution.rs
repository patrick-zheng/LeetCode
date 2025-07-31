pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let len1 = num1.len();
        let len2 = num2.len();
        let mut result = vec![0; len1 + len2];
        let bytes1 = num1.as_bytes();
        let bytes2 = num2.as_bytes();

        for i in (0..len1).rev() {
            for j in (0..len2).rev() {
                let mul = (bytes1[i] - b'0') as usize * (bytes2[j] - b'0') as usize;
                let p1 = i + j;
                let p2 = i + j + 1;
                let sum = mul + result[p2];

                result[p2] = sum % 10;
                result[p1] += sum / 10;
            }
        }

        let mut result_str = String::new();
         let mut leading_zero = true;
        for &digit in &result {
            if digit != 0 {
                leading_zero = false;
            }
            if !leading_zero {
                result_str.push((digit as u8 + b'0') as char);
           }
        }

        if result_str.is_empty() {
            "0".to_string()
        } else {
            result_str
        }
    }
}
