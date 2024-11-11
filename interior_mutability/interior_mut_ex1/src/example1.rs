// First lets create a node struct that will be our generic representation of a node.
// Lets instantiate a node and print its value
#[allow(dead_code)]
#[derive(Debug)]
struct Node{
    value: i32,
    neighbours: Vec<Node>
}

#[allow(dead_code)]
pub fn run(){

    let a = Node {
        value: 1,
        neighbours: vec![],
    };

    dbg!(a);

    
}


 