use std::io::{stdin, Read};

use day_06::node::Node;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut tree: Node = Node::build_tree(&input);
}