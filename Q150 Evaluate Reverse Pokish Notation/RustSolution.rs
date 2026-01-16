pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for t in tokens {
            match t.as_str() {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let res = match t.as_str() {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b, // Rust integer division truncates toward 0
                        _ => unreachable!(),
                    };
                    stack.push(res);
                }
                _ => {
                    stack.push(t.parse::<i32>().unwrap());
                }
            }
        }

        *stack.last().unwrap()
    }
}
