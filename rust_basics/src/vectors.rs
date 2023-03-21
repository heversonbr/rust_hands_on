/// ## Using Vectors in Rust
#[allow(dead_code)]
#[allow(unused_doc_comments)]
// I use allow(dead_code) to allow me to import all the modules in the `create` (my main.rs file)
// I use allow(unused_doc_comments) to avoid the 'warning: unused doc comment'


pub fn run(){
    /// A vector is a resizable array.
    ///     There are 3 different ways of declaring a vector in RUST.
    ///         . Using the new function:
    ///           Example: `let values: Vec<u32> = Vec::new();`  
    ///                     in order to put values in this vector we need to make it mutable => 
    ///                     `let mut values: Vec<u32> = Vec.new();`
    ///         . We can also use the vec! macro:
    ///           let mut values: Vec<i32>  = vec![1,2,3,4,5];
    ///          
    
    let mut numbers: Vec<i32>  = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // Reading elements in a vector: 
    // There are two ways to reference a value stored in a vector: 
    //       via indexing or using the get method.
    // get a single value using the index
    // using index gives us a reference to the element at the index value. 
    let element = &numbers[1];
    println!("Get element at index 1 (using index): {}", element);
    println!("Get element at index 0 (using index): {}", numbers[0]);

    // get a single value using get method:
    // When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match
    println!("Get value of a vector using get method: {:?}", numbers.get(3));

    // The reason Rust provides these two ways to reference an element is so you can choose how the program behaves when you try to use an index value outside the range of existing elements


    // get array length (number of elements in the vector)
    println!("Get Vector length: {}", numbers.len());

    // the capacity is the number of possible elements in the vector, without re-allocating it 
    println!("Get Vector capacity: {}", numbers.capacity());

    // reassigning values to the vector
    numbers[2] = 20;
    println!("{:?}", numbers);


    // Add elements in the Vector
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);
    println!("Get Vector length: {}", numbers.len());

    // Remove elements from the vector
    numbers.pop();
    println!("Vector pop {:?}", numbers);

    // arrays are stack allocated 
    println!("Vector_1 occupies {} bytes", std::mem::size_of_val(&numbers));
    let other_numbers: Vec<i32>  = vec![1];
    println!("Vector_2 occupies {} bytes", std::mem::size_of_val(&other_numbers));

    // A slice is a dynamically sized type representing a 'view' into a sequence of elements of type T. 
    // The slice type is written as [T].
    // Slice types are generally used through pointer types.
    // get slice of an array
    let slice1: &[i32] = &numbers[0..2];
    println!("Vector Slice 1: {:?} ", slice1);
    // another slice
    let slice2: &[i32] = &numbers[0..3];
    println!("Vector Slice 2: {:?} ", slice2);


    // Loop through vector items
    for x in numbers.iter(){
        println!("Number {} ", x);
    }

    // Loop and mutate values
    for elem in numbers.iter_mut(){
        *elem = *elem * 2;  // simplified to:   *elem *= 2
    }
    println!("Vector x2:  {:?}", numbers);




}