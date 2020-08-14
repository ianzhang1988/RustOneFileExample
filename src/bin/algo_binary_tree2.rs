use std::cmp::Ordering;

#[derive(Debug)]
enum NodeType<T: Ord> {
    Node{
        v:T,
        n:u32,
        l:Box<NodeType<T>>,
        r:Box<NodeType<T>>
    },
    Null,
}

use NodeType::Null;
use NodeType::Node;


impl <T: Ord> NodeType<T> {
    fn new()->NodeType<T> {
        Null
    }

    fn insert(&mut self, ov:T) {
        match self {
            &mut Node{ ref v, ref mut n, ref mut l, ref mut r } => {
                match v.cmp(&ov) {
                    Ordering::Less=> {
                        l.insert(ov);
                    },
                    Ordering::Greater=>{
                        r.insert(ov);
                    },
                    _ => {return}
                }
                *n = l.size()+r.size()+1;
            },
            Null => {
                *self = Node{v:ov, n:1, l:Box::new(Null), r:Box::new(Null)}
            }
        }
    }

    fn size(&self)-> u32 {
        match self {
            Node {n, ..} => *n,
            Null => 0
        }
    }
}

struct RefNodeIterator<T: Ord> {
    stack: Vec<Node<T>>,
    next: Option<T>,
}

impl <T: Ord> IntoIterator for &NodeType<T>{
    type Item= &T;
    type IntoIter = RefNodeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {

    }
}




fn main() {
    let mut b_tree = NodeType::<u32>::new();
    b_tree.insert(2);
    b_tree.insert(0);
    b_tree.insert(1);
    b_tree.insert(3);
    b_tree.insert(4);

    println!("{:?}", b_tree);
}