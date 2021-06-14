pub struct Stack<T> {
    data: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self { Self { data: Vec::new() } }

    pub fn push(&mut self, t: T) { self.data.push(t); }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.pop() {
            Some(t) => Some(t),
            None => None
        }
    }

    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod test {
    use crate::stack::*;

    #[test]
    fn push_and_pop() {
        let u32_data: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let char_data = &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'];

        let (mut stack1, mut stack2) = (Stack::new(), Stack::new());

        for data in u32_data {
            stack1.push(data);
        }
        for data in u32_data.into_iter().rev() {
            assert_eq!(stack1.pop().unwrap(), data);
        }

        for data in char_data {
            stack2.push(data);
        }
        for data in char_data.into_iter().rev() {
            assert_eq!(stack2.pop().unwrap(), data);
        }
    }
}