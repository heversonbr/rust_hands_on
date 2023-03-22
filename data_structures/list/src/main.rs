mod list;

#[allow(dead_code)]
fn main(){
   
   let mut mylist = list::SinglyLinkedList::<usize>::new();


   mylist.insert_front(1);
   mylist.insert_front(2);
   mylist.insert_front(3);
   mylist.insert_front(4);
   print!{"List length: {:?}\n", mylist.len()};


   mylist.show();

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






