// In this second example, lets create two other nodes.
//
// 
#[allow(dead_code)]
#[derive(Debug)]
struct Node{
    value: i32,
    neighbours: Vec<Node>
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn run(){

    let a = Node {
        value: 1,
        neighbours: vec![],
    };

    // lets create a node B that points to node A 
    let b = Node {
        value: 5,
        neighbours: vec![a],
    };
    // if we try to create another node B as above,
    // we will have an error because we moved the ownership of the value 'a' 
    // to 'b' and we try to use this value again below.
    // uncomment the next line to see the error. 

    //dbg!(a);
    //dbg!(b);

    // see the 'example 3' next. 
    
}


 