#[cfg(test)]
// the '#[cfg(test)]' attribute tells Rust to include 
// this module only when running tests
mod my_tests {

   // import the necessary modules here when necessary.
   use crate::list;
   use crate::list::SinglyLinkedList;

   // then, your test functions using the '#[test]' attribute
    #[test]
    fn create_list() {
      let mylist = SinglyLinkedList::<usize>::new();
      assert_eq!(mylist.len(), 0);
    }
    
   #[test]
   fn insert_node() {
      let mut mylist = list::SinglyLinkedList::<usize>::new();
      mylist.insert_front(4);
      if let Some(last_node) = mylist.get_last() {
         assert_eq!(*(last_node).get_data(), 4);
      }
   }
}