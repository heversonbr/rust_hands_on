// In this forth example, we want to update the values corresponding to each of our nodes. 
// 

#[allow(dead_code)]
#[derive(Debug)]
struct Node<'a>{
    value: i32,
    neighbours: Vec<&'a Node<'a>>    

}

// for that, lets create a function that adds one to each of the values (for the local node and its neighbours)
// however, if we try to create this function by passing either a reference to a node
// or, the ownership of the node , as shown below
// in both ways we wont be able to change the value of each neighbour 
//
//fn add_one(node: Node){
//   node.value = node.value + 1; 
//   for neighbour in node.neighbours {
//       add_one(&neighbour);
//   }
//}
//fn add_one(node: &Node){
//   node.value = node.value + 1; 
//   for neighbour in node.neighbours {
//       add_one(&neighbour);
//   }
//}

// the situation above shows exactly a situation where there's nothing unsafe concerning the memory
// there's no race conditions, there's no potential tangling refereces
// but the borrow checker does not allow us to change the value of the nodes.   
// In order to fix that we will need to use the Interior Mutability
// now, check the 'example 5'. 

#[allow(dead_code)]
pub fn run(){ 

    let a = Node {
        value: 1,
        neighbours: vec![],
    };

    let b = Node {
        value: 5,
        neighbours: vec![&a],
    };

    let c = Node {
        value: 15,
        neighbours: vec![&a],
    };
   
 
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);


   
    
}


 