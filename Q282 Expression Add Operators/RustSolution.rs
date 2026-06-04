pub struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let num = num.into_bytes();
        let n = num.len();
        let target = target as i64;
        let mut result = Vec::new();
        let mut path: Vec<u8> = Vec::with_capacity(n * 2);

        fn try_push(
            num: &[u8],
            n: usize,
            target: i64,
            result: &mut Vec<String>,
            path: &mut Vec<u8>,
            curr_val: i64,
            last_operand: i64,
            started: bool,
            value: i64,
            digit_start: usize,
            digit_end: usize,
            next_index: usize,
        ) {
            if !started {
                let mark = path.len();
                path.extend_from_slice(&num[digit_start..=digit_end]);
                dfs(
                    num,
                    n,
                    target,
                    result,
                    path,
                    value,
                    value,
                    true,
                    next_index,
                );
                path.truncate(mark);
                return;
            }

            let branches: [(u8, i64, i64); 3] = [
                (b'+', curr_val + value, value),
                (b'-', curr_val - value, -value),
                (
                    b'*',
                    curr_val - last_operand + last_operand * value,
                    last_operand * value,
                ),
            ];

            for (op, new_val, new_last) in branches {
                let mark = path.len();
                path.push(op);
                path.extend_from_slice(&num[digit_start..=digit_end]);
                dfs(num, n, target, result, path, new_val, new_last, true, next_index);
                path.truncate(mark);
            }
        }

        fn dfs(
            num: &[u8],
            n: usize,
            target: i64,
            result: &mut Vec<String>,
            path: &mut Vec<u8>,
            curr_val: i64,
            last_operand: i64,
            started: bool,
            index: usize,
        ) {
            if index == n {
                if curr_val == target {
                    result.push(String::from_utf8(path.clone()).unwrap());
                }
                return;
            }

            if num[index] == b'0' {
                try_push(
                    num,
                    n,
                    target,
                    result,
                    path,
                    curr_val,
                    last_operand,
                    started,
                    0,
                    index,
                    index,
                    index + 1,
                );
                return;
            }

            let mut value = 0i64;
            for end in index..n {
                if end > index && num[index] == b'0' {
                    break;
                }
                value = value * 10 + (num[end] - b'0') as i64;
                try_push(
                    num,
                    n,
                    target,
                    result,
                    path,
                    curr_val,
                    last_operand,
                    started,
                    value,
                    index,
                    end,
                    end + 1,
                );
            }
        }

        dfs(&num, n, target, &mut result, &mut path, 0, 0, false, 0);
        result
    }
}
