
#[allow(dead_code)]
pub fn run(){

    // Compound types can group multiple values into one type. 
    // Rust has two primitive compound types: TUPLES and ARRAYS.
    // ---------------------------------------------------------
    // Arrays:  an array is a fixed-size collection of elements of the same type.
    // Fixed size at compile-time (unlike other languages)
    // The elements can be mutable or immutable, depending on how the array is declared.
    //
    // One way of declaring an array is a list with each element, i.e., [x, y, z] 
    // Example: 
    {
        let number: [i32; 5] = [1,2,3,4,5];    // Array of 5 integers
        println!("{:?}", number);
        // NOTE: the following wont work
        // let not_allowed_array = [];
        // The reson is that we need to knwow the size 
        // the array will use in memory at compilation time
        // Key point: Arrays are stack-allocated, and all the elements are stored in a contiguous block
        //            their size cannot change after they are created
        //            therefore they are well adapted for Fixed-size collections. 
    }
    // Memory layout:   the elements are stored in a contiguous block of memory on the stack
    // Ex: 
    // let arr = [1, 2, 3, 4]; // Array of 4 integers
    //
    //  Stack:
    //  +----+----+----+----+
    //  |  1 |  2 |  3 |  4 |
    //  +----+----+----+----+

    {
        // Another way of declaring an array is:  array_name: [data_type; array lenght]
        // or more formally a repeat expression [T; N]
        // where T is the type of the elements, and N is the size of the array.

        let mut array_numbers = [3;5];
        // the array will have 5 elements with value 3

        // accessing a single value
        println!("Printing a single value {}", array_numbers[0]);
    }
    
    {   // Reassigning values:
        let mut numbers: [i32; 5] = [ 1,2,3,4,5 ];
        println!("array_numbers: {:?}", numbers);
        numbers[2] = 20;
        println!("array_numbers after change: {:?}", numbers);

        // get array length 
        println!("Array length: {}", numbers.len());

        // arrays are stack allocated 
        println!("Array occupies {} bytes", std::mem::size_of_val(&numbers)); 
        let new_numbers: [i32; 3] = [1,2,3];
        println!("Array occupies {} bytes", std::mem::size_of_val(&new_numbers));
    }

    // arrays are more useful when you know the number of elements will not need to change.
    // For example, if you were using the names of the month in a program, 
    // you would probably use an array rather than a vector because you know it will always contain 12 elements:
    // let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];



    {   // slices
        // NOTE: 
        // A slice is a kind of reference, so  it does not have ownership of the value.
        // let you reference a contiguous sequence of elements in a collection 
        // rather than the whole collection
        // the size for values of type `str` cannot be known at compilation time
        
        let mut numbers: [i32; 5] = [ 1,2,3,4,5 ];
        // get slice of an array
        let slice1: &[i32] = &numbers[0..2];
        println!("Slice1: {:?} ", slice1);
        // the slice1 is a simple reference on the stack 
        // that points to the element 0 and has lenght of 2 , definied by the range 0..2

        // get another slice , this is another reference (pointer created on the stack)
        let slice2: &[i32] = &numbers[0..3];
        println!("Slice2: {:?} ", slice2);
    }


}