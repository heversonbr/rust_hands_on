mod list;

#[allow(dead_code)]
fn main(){
   
   let mut mylist = list::SinglyLinkedList::<usize>::new();

   mylist.insert_front(1);
   print!{"List length: {:?}\n", mylist.len()};
   mylist.insert_front(2);
   print!{"List length: {:?}\n", mylist.len()};
   mylist.insert_front(3);
   print!{"List length: {:?}\n", mylist.len()};
   mylist.insert_front(4);
   print!{"List length: {:?}\n", mylist.len()};


   mylist.show();
   //println!("{:#?}" , &mylist);
   //dbg!(&mylist);
   

  
}






