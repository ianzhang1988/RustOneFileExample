use std::cmp::Ordering;
use std::cell::{Cell,RefCell};

#[derive(Debug)]
enum NodeType<T: Ord> {
    Node{
        v:T,
        n:u32,
        l:Box<RefCell<NodeType<T>>>,
        r:Box<RefCell<NodeType<T>>>
    },
    Null,
}

use NodeType::Null;
use NodeType::Node;
use std::borrow::{Borrow, BorrowMut};
use std::ops::DerefMut;


impl <T: Ord> NodeType<T> {
    fn new()->NodeType<T> {
        Null
    }

    fn insert(&mut self, ov:T) {
        match self {
            &mut Node{ ref v, ref mut n, ref mut l, ref mut r } => {
                match v.cmp(&ov) {
                    Ordering::Less=> {
                        (**r).borrow_mut().deref_mut().insert(ov);
                    },
                    Ordering::Greater=>{
                        (**l).borrow_mut().deref_mut().insert(ov);
                    },
                    _ => {return}
                }
                *n = (**l).borrow_mut().deref_mut().size()+(**r).borrow_mut().deref_mut().size()+1;
            },
            Null => {
                *self = Node{v:ov, n:1, l:Box::new(RefCell::new(Null)), r:Box::new(RefCell::new(Null))}
            }
        }
    }

    fn _insert(&mut self, node: Self) {
        //

        match self {
            &mut Node{ ref v, ref mut n, ref mut l, ref mut r } => {
                if let Node { v:vt, ..} = &node {
                    match v.cmp(&vt) {
                        Ordering::Less=> {
                            (**r).borrow_mut().deref_mut()._insert(node);
                        },
                        Ordering::Greater=>{
                            (**l).borrow_mut().deref_mut()._insert(node);
                        },
                        _ => {return}
                        }
                    *n = (**l).borrow_mut().deref_mut().size()+(**r).borrow_mut().deref_mut().size()+1;
                }
                else{
                    panic!("no going to happen");
                }
            },
            Null => {
                *self = node
            }
        }
    }

    fn size(&self)-> u32 {
        match self {
            Node {n, ..} => *n,
            Null => 0
        }
    }

    // fn get(&mut self, ov:T) -> Option<T> {
    //     match self {
    //         Node {ref v, l, r,..} => {
    //             match v.cmp( & ov) {
    //                 Ordering::Less => {
    //                     return r.get(ov);
    //                 },
    //                 Ordering::Greater =>{
    //                     return l.get(ov);
    //                 },
    //                 Ordering::Equal => {
    //                     return Some(ov);
    //                 }
    //             }
    //         },
    //         Null => None,
    //     }
    // }

    fn delete(&mut self, ov:T) -> bool{
        // let root = &self;
        if let Some((l,r)) = self._delete(ov) {

            if let Node {..} = l {
                self._insert(l);
            }

            if let Node {..} = r {
                self._insert(r);
            }

            true
        }
        else {
            false
        }
    }

    fn _delete(&mut self, ov:T) -> Option<(NodeType<T>, NodeType<T>)> {
        match self {
            // &mut Node {ref v, ref mut l, ref mut r,..} => {
            Node {ref v, l, r,..} => {
                match v.cmp( & ov) {
                    Ordering::Less => {
                        return (**r).borrow_mut().deref_mut()._delete(ov);
                    },
                    Ordering::Greater =>{
                        return (**l).borrow_mut().deref_mut()._delete(ov);
                    },
                    Ordering::Equal => {
                        let ln = (**l).replace(Null);
                        let rn = (**r).replace(Null);
                        *self = Null;
                        Some((ln,rn))
                        // None
                    }
                }
            },
            Null => None,
        }
    }
}


// #[derive(Debug)]
// struct NodeIteratorItem<'a,T: Ord + 'a> {
//     node_ref: &'a NodeType<T>,
//     ban_left: Cell<bool> // is there a way to avoid cell? // yes, use self.stack.last_mut() see comment below
// }
//
// struct RefNodeIterator<'a,T: Ord + 'a> {
//     stack: Vec<NodeIteratorItem<'a, T>>,
//     // next: Option<T>,
// }
//
// impl<'a, T: Ord + 'a + std::fmt::Display + std::fmt::Debug> Iterator for RefNodeIterator<'a, T> {
//     type Item = &'a T;
//
//     fn next(&mut self) -> Option<&'a T> {
//         // println!("next 1 ---------------------------------");
//
//         // let Some(xxx) = self.stack.last_mut() { this can avoid Cell
//         if let Some( NodeIteratorItem{ node_ref, ban_left}) = self.stack.last() {
//
//             let last = node_ref;
//
//             // println!("next 1.1 {:?}", self.stack);
//             // println!("next 1.1 node {:?}", self.stack.last());
//
//             match last {
//                 Node{ l, r, v, ..} => {
//                     // println!("next 1.1.1 {}", v);
//                     match ( l.borrow(),  r.borrow()) {
//                         (Node {..}, Node {..})=>{
//                                 // println!("next 1.2");
//
//                                 if ban_left.get() {
//                                     self.stack.pop();
//                                     self.stack.push(NodeIteratorItem{ node_ref:r.borrow(), ban_left: Cell::new(false) });
//                                     return Some(v);
//                                 }
//
//                                 ban_left.set(true);
//                                 self.stack.push(NodeIteratorItem{ node_ref:l.borrow(), ban_left: Cell::new(false) });
//
//                                 return self.next();
//                             },
//                         (Node {..}, Null)=>{
//                                 // println!("next 1.2 {}", ban_left.get());
//
//                                 if ban_left.get() {
//                                     // println!("next 1.2.1");
//                                     self.stack.pop();
//                                     return Some(v);
//                                 }
//
//                                 // println!("next 1.2.2");
//
//                                 ban_left.set(true);
//                                 self.stack.push(NodeIteratorItem{ node_ref:l.borrow(), ban_left: Cell::new(false)});
//                                 return self.next();
//                             },
//                         (Null, Node {..})=>{
//                                 // println!("next 1.3");
//                                 self.stack.pop();
//                                 self.stack.push(NodeIteratorItem{ node_ref:r.borrow(), ban_left: Cell::new(false)});
//                                 return Some(v);
//                             },
//                         (Null, Null)=>{
//                                 // println!("next 1.4");
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
//         // println!("next 2");
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
//         stack.push(NodeIteratorItem{ node_ref:self, ban_left: Cell::new(false)});
//         RefNodeIterator{ stack: stack }
//     }
// }

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

    // forgot about xxx_mut, shame on me...
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

    b_tree.delete(3);

    println!("+++++++++++++++++++++");

    println!("{:?}", &b_tree);



    // let it = &mut b_tree.into_iter();
    // it.next();
    // it.next();
    // it.next();
    // it.next();
    // it.next();
    // it.next();
    //
    // for i in &b_tree {
    //     println!("+++++++++ {}", i);
    // }
    //
    // let mut b_tree = NodeType::<u32>::new();
    // b_tree.insert(8);
    // b_tree.insert(4);
    // b_tree.insert(2);
    // b_tree.insert(5);
    // b_tree.insert(1);
    // b_tree.insert(3);
    // b_tree.insert(6);
    // b_tree.insert(7);
    // b_tree.insert(12);
    // b_tree.insert(10);
    // b_tree.insert(14);
    // b_tree.insert(9);
    // b_tree.insert(11);
    // b_tree.insert(13);
    // b_tree.insert(15);
    //
    // println!("{:?}", &b_tree);
    //
    // for i in &b_tree {
    //     println!("+++++++++ {}", i);
    // }
    //
    // println!("{}", b_tree.get(3).unwrap_or(0));
    // println!("{}", b_tree.get(20).unwrap_or(0));
}