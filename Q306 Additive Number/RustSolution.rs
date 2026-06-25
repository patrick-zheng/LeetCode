pub struct Solution;

fn add_strings(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    let mut i = a.len() as i32 - 1;
    let mut j = b.len() as i32 - 1;
    let mut carry = 0;

    while i >= 0 || j >= 0 || carry > 0 {
        let mut sum = carry;
        if i >= 0 {
            sum += (a[i as usize] - b'0') as i32;
            i -= 1;
        }
        if j >= 0 {
            sum += (b[j as usize] - b'0') as i32;
            j -= 1;
        }
        res.push((sum % 10) as u8 + b'0');
        carry = sum / 10;
    }
    res.reverse();
    res
}

fn valid_pair(num: &[u8], i: usize, j: usize) -> bool {
    let n = num.len();
    if num[0] == b'0' && i > 1 {
        return false;
    }
    if num[i] == b'0' && j > i + 1 {
        return false;
    }

    let mut a = num[..i].to_vec();
    let mut b = num[i..j].to_vec();
    let mut k = j;

    while k < n {
        let c = add_strings(&a, &b);
        if k + c.len() > n || &num[k..k + c.len()] != c.as_slice() {
            return false;
        }
        a = b;
        b = c;
        k += b.len();
    }
    true
}

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let bytes = num.as_bytes();
        let n = bytes.len();

        for j in 1..n {
            for i in 1..j {
                if valid_pair(bytes, i, j) {
                    return true;
                }
            }
        }
        false
    }
}
