struct Node<T> {
    data: T,
    next_node: Option<Box<Node<T>>>
}

pub struct LinkedList<T> {
    start_node: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    /// Constructs a new empty `LinkedList<T>`.
    pub fn new() -> Self {
        LinkedList { start_node: None }
    }

    /// Appends an element to the end of collection.
    pub fn add(&mut self, value: T) {
        let mut node_option = &mut self.start_node;

        loop {
            match node_option {
                Some(node) => { node_option = &mut node.next_node; },
                None => {
                    let node = Node { data: value, next_node: None};
                    *node_option = Some(Box::new(node));
                    break;
                }
            }
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        let mut len = 0;

        let mut current_node = &self.start_node;
        loop {
            match current_node {
                Some(node) => {
                    len += 1;
                    current_node = &node.next_node;
                },
                None => { break; }
            };
        }

        len
    }
}

impl<T> std::ops::Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut xp = &self.start_node;

        for _ in 0..index {
            xp = match xp {
                Some(node_box) => &node_box.next_node,
                None => panic!(
                    "Either list is empty or out-of-index error.")
            };
        }

        match xp {
            Some(node_box) => &node_box.data,
            None => panic!("There is no node.")
        }
    }
}

impl<T: Clone> std::ops::IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut xp = &mut self.start_node;

        for _ in 0..index {
            xp = match xp {
                Some(node_box) => &mut node_box.next_node,
                None => panic!(
                    "Either list is empty or out-of-index error.")
            };
        }

        match xp {
            Some(node_box) => &mut node_box.data,
            None => panic!("There is no node.")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::linked_list::*;
    use std::ops::Range;

    #[test]
    fn add_and_get() {
        let mut linked_list = LinkedList::new();

        let range: Range<usize> = 0..10;
        let i10: Vec<u8> = range.clone().map(|i| (i * 10) as u8).collect();

        for i in i10.clone() {
            linked_list.add(i);
        }

        for (i, i10) in range.zip(i10) {
            assert_eq!(linked_list[i], i10);
        }
    }

    #[test]
    fn index_mut() {
        let mut linked_list = LinkedList::new();

        let range: Range<usize> = 0..10;
        let i10: Vec<u8> = range.clone().map(|i| (i * 10) as u8).collect();
        let i12: Vec<u8> = i10.iter().map(|i| i + 2).collect();

        for i in i10.clone() {
            linked_list.add(i);
        }

        for i in range.clone() {
            linked_list[i] += 2;
        }

        for i in range {
            assert_eq!(linked_list[i], i12[i]);
        }
    }

    #[test]
    fn len() {
        let mut linked_list = LinkedList::new();

        assert_eq!(linked_list.len(), 0);

        let data1 = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let data2 = &[0, 1];

        for data in data1 {
            linked_list.add(data);
        }
        assert_eq!(linked_list.len(), data1.len());

        for data in data2 {
            linked_list.add(data);
        }
        assert_eq!(linked_list.len(), data1.len() + data2.len());
    }
}