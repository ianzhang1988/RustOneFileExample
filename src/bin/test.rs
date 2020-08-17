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
use std::borrow::Borrow;


impl <T: Ord> NodeType<T> {
    fn new()->NodeType<T> {
        Null
    }

    fn insert(&mut self, ov:T) {
        match self {
            &mut Node{ ref v, ref mut n, ref mut l, ref mut r } => {
                match v.cmp(&ov) {
                    Ordering::Less=> {
                        r.insert(ov);
                    },
                    Ordering::Greater=>{
                        l.insert(ov);
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

// struct RefNodeIterator<'a,T: Ord + 'a> {
//     stack: Vec<&'a NodeType<T>>,
//     // next: Option<T>,
// }
//
// impl<'a, T: Ord + 'a + std::fmt::Display + std::fmt::Debug> Iterator for RefNodeIterator<'a, T> {
//     type Item = &'a T;
//
//     fn next(&mut self) -> Option<&'a T> {
//         println!("next 1");
//         if let Some(last) = self.stack.last() {
//
//             println!("next 1.1 {:?}", self.stack);
//
//             match last {
//                 Node{ l, r, v, ..} => {
//                     println!("next 1.1.1 {}", v);
//                     match ( l.borrow(),  r.borrow()) {
//                         (Node {..}, _)=>{
//                                 println!("next 1.2");
//                                 self.stack.push(l.borrow());
//                                 self.next();  // dead loop
//                             },
//                         (Null, Node {..})=>{
//                                 println!("next 1.3");
//                                 self.stack.pop();
//                                 self.stack.push(r.borrow());
//                                 return Some(v);
//                             },
//                         (Null, Null)=>{
//                                 println!("next 1.4");
//                                 self.stack.pop();
//                                 return Some(v);
//                             },
//                     }
//
//                 },
//                 Null => return None
//             }
//         }
//
//         println!("next 2");
//         None
//     }
// }
//
// impl <'a, T: Ord + std::fmt::Display + std::fmt::Debug> IntoIterator for &'a NodeType<T>{
//     type Item= &'a T;
//     type IntoIter = RefNodeIterator<'a, T>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         println!("IntoIterator");
//         let mut stack = Vec::new();
//         stack.push(self);
//         RefNodeIterator{ stack: stack }
//     }
// }

#[derive(Debug)]
struct NodeIteratorItem<'a,T: Ord + 'a> {
    node_ref: &'a NodeType<T>,
    ban_left: bool // is there a way to avoid cell?
}

struct RefNodeIterator<'a,T: Ord + 'a> {
    stack: Vec<NodeIteratorItem<'a, T>>,
    // next: Option<T>,
}

impl<'a, T: Ord + 'a + std::fmt::Display + std::fmt::Debug> Iterator for RefNodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // println!("next 1 ---------------------------------");
        if let Some( NodeIteratorItem{ node_ref, ban_left}) = self.stack.last_mut() {

            let last = node_ref;

            // println!("next 1.1 {:?}", self.stack);
            // println!("next 1.1 node {:?}", self.stack.last());

            match last {
                Node{ l, r, v, ..} => {
                    // println!("next 1.1.1 {}", v);
                    match ( l.borrow(),  r.borrow()) {
                        (Node {..}, Node {..})=>{
                                // println!("next 1.2");

                                if *ban_left {
                                    self.stack.pop();
                                    self.stack.push(NodeIteratorItem{ node_ref:r.borrow(), ban_left: false });
                                    return Some(v);
                                }

                                *ban_left = true;
                                self.stack.push(NodeIteratorItem{ node_ref:l.borrow(), ban_left: false });

                                return self.next();
                            },
                        (Node {..}, Null)=>{
                                // println!("next 1.2 {}", ban_left.get());

                                if *ban_left {
                                    // println!("next 1.2.1");
                                    self.stack.pop();
                                    return Some(v);
                                }

                                // println!("next 1.2.2");

                                *ban_left = true;
                                self.stack.push(NodeIteratorItem{ node_ref:l.borrow(), ban_left: false});
                                return self.next();
                            },
                        (Null, Node {..})=>{
                                // println!("next 1.3");
                                self.stack.pop();
                                self.stack.push(NodeIteratorItem{ node_ref:r.borrow(), ban_left: false});
                                return Some(v);
                            },
                        (Null, Null)=>{
                                // println!("next 1.4");
                                self.stack.pop();
                                return Some(v);
                            },
                    }

                },
                Null => return None
            }
        }

        // println!("next 2");
        None
    }
}

impl <'a, T: Ord + std::fmt::Display + std::fmt::Debug> IntoIterator for &'a NodeType<T>{
    type Item= &'a T;
    type IntoIter = RefNodeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        println!("IntoIterator");
        let mut stack = Vec::new();
        stack.push(NodeIteratorItem{ node_ref:self, ban_left: false});
        RefNodeIterator{ stack: stack }
    }
}

struct Foo{
    value:u32,
    flag: bool
}

struct Bar{
    stack: Vec<Foo>
}


fn main() {

    let mut b = Bar{ stack:Vec::new() };
    b.stack.push(Foo {value:1, flag:true});
    b.stack.push(Foo {value:2, flag:false});

    let last = b.stack.last_mut().unwrap();
    last.value=10;
    last.flag=true;

    let mut b_tree = NodeType::<u32>::new();
    b_tree.insert(2);
    b_tree.insert(0);
    b_tree.insert(1);
    b_tree.insert(3);
    b_tree.insert(4);

    println!("{:?}", &b_tree);

    // for i in &b_tree {
    //     println!("+++++++++ {}", i);
    // }

    // let it = &mut b_tree.into_iter();
    // it.next();
    // it.next();
    // it.next();
    // it.next();
    // it.next();
    // it.next();

    let mut b_tree = NodeType::<u32>::new();
    b_tree.insert(8);
    b_tree.insert(4);
    b_tree.insert(2);
    b_tree.insert(5);
    b_tree.insert(1);
    b_tree.insert(3);
    b_tree.insert(6);
    b_tree.insert(7);
    b_tree.insert(12);
    b_tree.insert(10);
    b_tree.insert(14);
    b_tree.insert(9);
    b_tree.insert(11);
    b_tree.insert(13);
    b_tree.insert(15);

    println!("{:?}", &b_tree);

    for i in &b_tree {
        println!("+++++++++ {}", i);
    }
}