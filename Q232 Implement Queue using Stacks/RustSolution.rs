struct MyQueue {
    in_stk: Vec<i32>,
    out_stk: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            in_stk: Vec::new(),
            out_stk: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.in_stk.push(x);
    }

    fn move_to_out(&mut self) {
        if !self.out_stk.is_empty() {
            return;
        }
        while let Some(v) = self.in_stk.pop() {
            self.out_stk.push(v);
        }
    }

    fn pop(&mut self) -> i32 {
        self.move_to_out();
        self.out_stk.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.move_to_out();
        *self.out_stk.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.in_stk.is_empty() && self.out_stk.is_empty()
    }
}
