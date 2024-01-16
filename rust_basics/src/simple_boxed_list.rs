
#[allow(dead_code)]
pub fn run() {

    // Node struct that will be stored on the list
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    // Create a linked list of nodes
    let mut head:Option<Box<Node>> = None;

    // Create nodes, each with a different i32 value inside
    for node_value in 1..11 {
        // create a new node
        let mut new_node = Box::new(Node{value: node_value, next: None});
        // check if head is none, add new node as first node if so, 
        // add node in the beginning of the list, otherwise
        println!("Creating node: {:?} and adding it into the beginning of the list.", new_node);
        if head.is_none() {
            head = Some(new_node);
        }else {
            new_node.next = head.take();
            head = Some(new_node);
        }
    }

    println!("\nFull List : {:?} \n", &head);

    // Traverse the linked list and print the node values
    let mut aux_node = &head;
    
    while let Some(current_node) = aux_node {
        println!("Current node: {:?}", &current_node.value);
        aux_node = &current_node.next;
    }

}
