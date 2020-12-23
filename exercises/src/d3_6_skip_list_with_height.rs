use std::{
    cell::RefCell,
    fmt::{Display, Write},
    rc::Rc,
};

type Rcc<T> = Rc<RefCell<T>>;

struct Node<T: PartialOrd> {
    right: Option<Rcc<Node<T>>>,
    down: Option<Rcc<Node<T>>>,
    data: Rc<T>,
}

pub struct SkipList<T: PartialOrd> {
    list: Vec<Node<T>>,
}

fn rcc<T: PartialOrd>(node: Node<T>) -> Rcc<Node<T>> {
    Rc::new(RefCell::new(node))
}

impl<T: PartialOrd> Node<T> {
    pub fn new(data: Rc<T>, right: Option<Rcc<Node<T>>>, down: Option<Rcc<Node<T>>>) -> Self {
        Node { right, down, data }
    }

    pub fn insert(&mut self, data: T) -> Option<Rcc<Node<T>>> {
        // If there is a child on the right, and the data is greater than it, recursively insert on
        // the right.
        //
        if let Some(right) = &self.right {
            let mut right = right.borrow_mut();

            if data > *right.data {
                return right.insert(data);
            }
        }

        if let Some(down) = &self.down {
            let inserted_node = down.borrow_mut().insert(data);

            if let Some(inserted_node) = inserted_node {
                if rand::random() {
                    let data = &inserted_node.borrow().data;
                    let down = Some(Rc::clone(&inserted_node));

                    return self.insert_to_right(Rc::clone(data), down);
                }
            }

            return None;
        }

        self.insert_to_right(Rc::new(data), None)
    }

    // Returns the new node.
    //
    fn insert_to_right(&mut self, data: Rc<T>, down: Option<Rcc<Node<T>>>) -> Option<Rcc<Node<T>>> {
        let extracted_right = self.right.take();
        let new_node = rcc(Node::new(data, extracted_right, down));
        self.right = Some(Rc::clone(&new_node));
        Some(new_node)
    }
}

impl<T: PartialOrd + Display> Node<T> {
    fn print_row<U: Write>(&self, mut writer: U) -> U {
        write!(writer, "{}", self.data).unwrap();

        if let Some(right) = &self.right {
            write!(writer, ",").unwrap();
            right.borrow().print_row(writer)
        } else {
            writer
        }
    }
}

impl<T: PartialOrd + Display> SkipList<T> {
    fn print<U: Write>(&self, mut writer: U) -> U {
        if self.list.is_empty() {
            write!(writer, "Empty skip list!").unwrap();
            return writer;
        }

        for node in &self.list {
            write!(writer, "\n").unwrap();
            writer = node.print_row(writer);
        }

        writer
    }
}

impl<T: PartialOrd> SkipList<T> {
    pub fn new() -> Self {
        SkipList { list: vec![] }
    }

    pub fn insert(&mut self, data: T) {
        if self.list.is_empty() {
            self.list.push(Node::new(Rc::new(data), None, None));
            return;
        }

        for (i, node) in self.list.iter_mut().enumerate().rev() {
            if data > *node.data {
                let inserted_node = node.insert(data);

                if let Some(inserted_node) = inserted_node {
                    self.loop_up(inserted_node, i + 1)
                }

                return;
            }
        }

        let new_node = Node::new(Rc::new(data), None, None);

        self.replace_and_loop_up(new_node, 0);
    }

    fn loop_up(&mut self, down: Rcc<Node<T>>, n: usize) {
        if rand::random() {
            return;
        }

        let data = Rc::clone(&down.borrow().data);
        let new_node = Node::new(data, None, Some(down));

        if n >= self.list.len() {
            self.list.push(new_node);
            return;
        }

        self.replace_and_loop_up(new_node, n);
    }

    fn replace_and_loop_up(&mut self, mut node: Node<T>, n: usize) {
        std::mem::swap(&mut node, &mut self.list[n]);

        let node = rcc(node);
        self.list[n].right = Some(Rc::clone(&node));

        self.loop_up(node, n + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn test_list() -> SkipList<i32> {
        let mut list = SkipList::new();

        list.insert(4);
        list.insert(6);
        list.insert(77);
        list.insert(84);
        list.insert(23);
        list.insert(1);

        list
    }

    #[test]
    fn test_insert() {
        let list = test_list();

        // true
        // false
        // true
        // true
        // false
        // false
        // true

        let rand_byte: u8 = rand::random();

        let actual_representation = list.print(String::from(""));
        let expected_representation = indoc! {"

            1,4,6,23,77,84
            23,77,84
            77"};

        println!("AK: {}", actual_representation);
        assert_eq!(actual_representation, expected_representation);
    }
}
