

pub fn run() {
    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    // Create a linked list of nodes
    let mut head = Some(Box::new(Node { value: 1, next: None }));
    let mut tail = &mut head;
    for i in 2..10 {
        let node = Some(Box::new(Node { value: i, next: None }));
        tail.as_mut().unwrap().next = node;
        tail = &mut tail.as_mut().unwrap().next;
    }

    // Traverse the linked list and print the node values
    let mut current_node = &head;
    while let Some(node) = current_node {
        println!("Node value: {}", node.value);
        current_node = &node.next;
    }
}
