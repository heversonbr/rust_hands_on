// In this third example, we change the definition of the 
// neighbours vector to references in order to allow creating many references to a single node
// 
#[allow(dead_code)]
#[derive(Debug)]
struct Node<'a>{
    value: i32,
    neighbours: Vec<&'a Node<'a>>    
    // Now the neighbours vector holds references to nodes instead of owned values
    // we also add the required lifetimes
}

#[allow(dead_code)]
pub fn run(){ 

    let a = Node {
        value: 1,
        neighbours: vec![],
    };

    // and now we can create node B and C that point both to node A 
    let b = Node {
        value: 5,
        neighbours: vec![&a],
    };

    let c = Node {
        value: 15,
        neighbours: vec![&a],
    };
   
    // now we can print the 3 nodes correctly, 
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);

    // however, as we discussed before, it goes well if these nodes are read-only
    // at this point we can not change the values of each node. 
    // imagine that for some reason we need to update the values of each node. 
    // now, check the 'example 4'. 
    
}


 