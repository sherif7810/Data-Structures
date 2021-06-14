pub struct Stack<T> {
    data: Vec<T>
}

impl Stack<T> {
    pub fn new() -> Self { Self { data: Vec::new() } }

    pub fn push(&mut self, t: T) { self.data.push(t); }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.pop() {
            Some(t) => t,
            None => None
        }
    }

    pub fn len(&self) -> usize { self.data.len() }
}