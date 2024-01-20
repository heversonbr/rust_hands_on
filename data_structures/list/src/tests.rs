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
   fn insert_front() {
      let mut mylist = list::SinglyLinkedList::<usize>::new();
      mylist.push_front(4);
      // test if length is correct 
      assert_eq!(mylist.len(), 1);
      // test if the only inserted value is the head
      // here we use 'if let' expressions  as a shorter way to write 
      // the equivalent of a 'match' that only matches one case.
      if let Some(current_head) =  mylist.get_head() {
         let res = current_head.as_ref();
         let value = *res.get_data();

         assert_eq!(value, 4);
      }
   }

   #[test]
   fn check_get_last(){
      // create an empty list 
      let mut mylist = list::SinglyLinkedList::<usize>::new();
      // insert 4 elements: 1, 2, 3, 4 
      mylist.push_front(4);
      mylist.push_front(3);
      mylist.push_front(2);
      mylist.push_front(1);
      // here we use 'if let' expressions  as a shorter way to write 
      // the equivalent of a 'match' that only matches one case.
      if let Some(last_node) = mylist.get_last(){
         assert_eq!(*last_node.get_data(), 4);
      }
   }

   #[test]
   fn check_pop_front(){

      // create an empty list 
      let mut mylist = list::SinglyLinkedList::<usize>::new();
      // insert 4 elements: 1, 2, 3, 4 
      mylist.push_front(4);
      mylist.push_front(3);
      mylist.push_front(2);
      mylist.push_front(1);

      if let Some(first) = mylist.pop_front(){
         assert_eq!(first, 1);
      }
   }

   #[test]
   fn check_get_node() { 

      // create an empty list 
      let mut mylist = list::SinglyLinkedList::<usize>::new();
      // insert 4 elements: 1, 2, 3, 4 
      mylist.push_front(4);
      mylist.push_front(3);
      mylist.push_front(2);
      mylist.push_front(1);


      if let Some(node) = mylist.get_node(2){
         assert_eq!(*node.get_data(), 2);
      }
   }

}