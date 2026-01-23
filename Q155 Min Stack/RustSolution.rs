struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {

    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        if let Some(&(_, current_min)) = self.stack.last() {
            self.stack.push((val, val.min(current_min)));
        } else {
            self.stack.push((val, val));
        }
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}
