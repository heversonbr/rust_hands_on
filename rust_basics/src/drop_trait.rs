// The Drop trait is similar to the 'Deref' trait but used for DESTRUCTURING, 
// which Rust automatically implements by cleaning up resources that are no longer being used by a program. 
// So, the Drop trait is used on a pointer that stores the unused value, 
// and then deallocates the space in memory that the value occupied.
//To use the Drop trait, you’ll need to implement the drop() method with a mutable 
// reference that executes destruction for values that are no longer needed or are out of scope.

// This example shows how the Drop trait works:
// from :  https://blog.logrocket.com/smart-pointers-rust/
// improved example using a difference scope for the first variable


// We create a struct that represents the number of habitants we have in a small town:
struct Consensus  {
    small_town: i32
}

// then, we implement the Drop trait for our 'Consensus' struct 
// The drop function will be executed when a value goes out of scope. 
impl Drop for Consensus {
    fn drop(&mut self) {
            println!("This instance of Consensus has being dropped: {}", self.small_town);
}
}

fn main() {

    {
        println!("Created _first_instance of Consensus");
        let _first_instance = Consensus{small_town: 10};
        println!("End _first_instance scope");
    }   // first value out of scope

    println!("Created _second_instance of Consensus");
    let _second_instance = Consensus{small_town: 8};

    println!("Created _third_instance of Consensus");
    let _third_instance = Consensus{small_town: 6};

    println!("End main scope");
}
// second and third values out of scope