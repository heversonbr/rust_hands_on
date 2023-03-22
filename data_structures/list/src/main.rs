mod list;

#[allow(dead_code)]
fn main(){
   
   let mut mylist = list::SinglyLinkedList::<usize>::new();
   mylist.insert_front(1);
   mylist.insert_front(2);
   mylist.insert_front(3);
   mylist.insert_front(4);
   //print!{"List length: {:?}\n", mylist.len()};
   //mylist.show();

   mylist.insert_end(5);
   mylist.insert_end(6);
   mylist.show();

   // print last (uses if let pattern)
   //if let Some(last_node) = mylist.get_last() {
   //   println!("Last node data:  {:?}" , last_node.get_data());
   //}

   // print last (uses match pattern)
   //match mylist.get_last(){
   //   Some(last) => {
   //      // last is:  &mut Node<T> 
   //      println!(" {:?}" , *last.get_data());
   //   }
   //   None => { println!("last is None.") }
   //}

   // look for node with specific data 
   //let target_data = 2;
   //println!("Look for node with data {:?} ", target_data);
   //let tgt_node = mylist.get_node(target_data); 
   //match tgt_node {
   //   Some(found) => {
   //      println!("Target node found => {:?} ", found);
   //      println!("Target node found  => Node.data: {:?} ", found.get_data());
   //   }
   //   None => { println!("not found"); }
   //}

   // using let some pattern (only cares about positive result)
   //let target_data = 7;
   //println!("Look for node with data {:?} ", target_data);
   //if let Some(target_node) = mylist.get_node(target_data) {
   //      println!("Target node found => {:?} ", target_node);
   //      println!("Target node found  => Node.data: {:?} ", target_node.get_data());
   //}

  
}


#[test]
fn test_create_list() {
   let mylist = list::SinglyLinkedList::<usize>::new();
   assert_eq!(mylist.len(), 0);
}

#[test]
fn test_insert_node() {
   
   let mut mylist = list::SinglyLinkedList::<usize>::new();
   mylist.insert_front(4);
   if let Some(last_node) = mylist.get_last() {
      assert_eq!(*(last_node).get_data(), 4);
   }
   

}






