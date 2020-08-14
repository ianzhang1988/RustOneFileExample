use std::cell::Cell;

#[derive(Debug)]
enum NodeType<T> {
    Node(Box<Node<T>>),
    Null,
}

use NodeType::Null;
use std::borrow::{BorrowMut, Borrow};

#[derive(Debug)]
struct Node<T> {
    value: T,
    left: NodeType<T>,
    right:NodeType<T>,
}

#[derive(Debug)]
struct Foo {
    v1: u32,
    v2: u32,
    v3: String
}

/// update on Cell comment in bin/tcp_server_frame2.rs
///
/// Box is not immutable, a &mut T reference can change the data inside
/// so, what's the different between Cell and Box
/// &mut T is needed to change data in Box, on the other hand, only &T is needed to change data inside Cell
/// in other words, in Rust you can't use multiple &mut T reference to T, but some time you must change
/// data form multiple reference, that's what Cell is meant for.
///

fn box_vs_cell(){
    // data in Box can be change
    let mut b = Box::new(1);
    println!("{}", b);
    *b=2; // what..., i thought Box is immutable inside ...
    println!("{}", b);

    let mut b2 = Box::new(Foo{v1:1,v2:2,v3:"hello".to_string()});
    println!("{:?}", b2);
    b2.v1=5;
    println!("{:?}", b2);

    // let b3 = &mut b;
    // let b4 = &mut b;
    // **b3=4;
    // **b4=8;

    // let b3 = &mut b;
    //          ------ first mutable borrow occurs here
    // let b4 = &mut b;
    //          ^^^^^^ second mutable borrow occurs here
    // **b3=4;
    // ------ first borrow later used here

    let c = Cell::new(1);

    let mut c1 = &c;
    let mut c2 = &c;

    c1.borrow_mut().set(2);
    println!("{}", c.borrow().get());

    c2.borrow_mut().set(3);
    println!("{}", c.borrow().get());

}

fn main() {

    box_vs_cell();

    let mut root = Node{value:0, left: Null, right: Null };
    let node_1 = NodeType::Node(Box::new(Node{value:1, left: Null, right: Null }));
    let node_2 = NodeType::Node(Box::new(Node{value:2, left: Null, right: Null }));
    let node_3 = NodeType::Node(Box::new(Node{value:3, left: Null, right: Null }));

    root.left=node_1;
    root.right=node_2;

    // node_1.left=node_3;
    if let NodeType::Node(node) = &mut (root.left) {
        node.left = node_3;
    }

    println!("{:?}", root);



}