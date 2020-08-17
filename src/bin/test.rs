#![feature(box_patterns)]

struct Node<T: PartialOrd> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct NodeIterator<T: PartialOrd> {
    stack: Vec<Node<T>>,
    next: Option<T>,
}

impl<T: PartialOrd> IntoIterator for Node<T> {
    type Item = T;
    type IntoIter = NodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = Vec::new();

        let smallest = pop_smallest(self, &mut stack);

        NodeIterator { stack: stack, next: Some(smallest) }
    }
}

impl<T: PartialOrd> Iterator for NodeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(next) = self.next.take() {
            return Some(next);
        }

        if let Some(Node { value, right, .. }) = self.stack.pop() {
            if let Some(right) = right {
                let box right = right;
                self.stack.push(right);
            }
            return Some(value);
        }

        None
    }
}

fn pop_smallest<T: PartialOrd>(node: Node<T>, stack: &mut Vec<Node<T>>) -> T {
    let Node { value, left, right } = node;

    if let Some(left) = left {
        stack.push(Node { value: value, left: None, right: right });
        let box left = left;
        return pop_smallest(left, stack);
    }

    if let Some(right) = right {
        let box right = right;
        stack.push(right);
    }

    value
}

fn main() {
    let root = Node {
        value: 4,
        left: Some(Box::new(Node { value: 2,
            left: Some(Box::new(Node { value: 1, left: None, right: None })),
            right: Some(Box::new(Node { value: 3, left: Some(Box::new(Node { value: 10, left: None, right: None })), right: None }) )})),
        right: Some(Box::new(Node { value: 6,
            left: Some(Box::new(Node { value: 5, left: None, right: None })),
            right: Some(Box::new(Node { value: 7, left: None, right: None }) )}))
    };

    for t in root {
        println!("{}", t);
    }
}