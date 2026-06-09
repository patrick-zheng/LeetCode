struct PeekingIterator {
    iter: std::vec::IntoIter<i32>,
    peeked: Option<i32>,
}

impl PeekingIterator {
    fn new(mut iter: std::vec::IntoIter<i32>) -> Self {
        let peeked = iter.next();
        PeekingIterator { iter, peeked }
    }

    fn next(&mut self) -> i32 {
        let value = self.peeked.take().unwrap();
        self.peeked = self.iter.next();
        value
    }

    fn peek(&self) -> i32 {
        self.peeked.unwrap()
    }

    fn has_next(&self) -> bool {
        self.peeked.is_some()
    }
}
