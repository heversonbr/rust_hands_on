mod list;
mod tests;
// the compiler will look for the module 'list': 
// In the file src/list.rs          or,
// In the file src/list/mod.rs 
use list::SinglyLinkedList;
//use list::SinglyLinkedList;   allows for shortcuting the path and use 
//     SinglyLinkedList::<usize>::new();         instead of  
//     list::SinglyLinkedList::<usize>::new(); 

#[allow(dead_code)]
fn main(){

   let mut mylist = SinglyLinkedList::<usize>::new();
   println!("Inserting at the end...");
   mylist.push_end(3);
   mylist.push_end(4);
   mylist.push_end(5);
   mylist.push_end(6);
   // node(3) node(4) node(5) node(6) 

   mylist.show();

   println!("Inserting at the beginning...");
   mylist.push_front(2);
   mylist.push_front(1);
   // node(1) node(2) node(3) node(4) node(5) node(6)
   mylist.show();

   print!{"List length: {:?}\n", mylist.len()};

   // print last (uses if let pattern)
   // if let Some(last_node) = mylist.get_last() {
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

   // look for a node with specific data (uses match pattern)
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

   // search node using let some pattern (only cares about positive result)
   //let target_data = 2;
   //println!("Look for node with data {:?} ", target_data);
   //if let Some(target_node) = mylist.get_node(target_data) {
   //      println!("Target node found => {:?} ", target_node);
   //      println!("Target node found  => Node.data: {:?} ", target_node.get_data());
   //}


  // update node value using let some pattern
  //  println!("Updating data...");
  //  let target_data = 2;
  //  println!("Look for node with data {:?} ", target_data);
  //  if let Some(target_node) = mylist.get_node(target_data) {
  //        println!("Target node found => {:?} ", target_node);
  //        println!("Target node found  => Node.data: {:?} ", target_node.get_data());
  //        target_node.set_data(10);
  //  }
  //   mylist.show();

}


