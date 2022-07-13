// Arrays: fixed lenght data types. Fixed lists where elements are of the same kind. 

pub fn run(){

    // declare array 
    // => array_nme: [data_tyoe; array lenght]
    let number: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", number);

    // get the single value
    println!("Printing a single value {}", number[0]);

    // reassigning values:
    let mut numbers: [i32; 5] = [ 1,2,3,4,5 ];
    println!("{:?}", numbers);
    numbers[2] = 20;
    println!("{:?}", numbers);

    // get array length 
    println!("Array length: {}", numbers.len());

    // arrays are stack allocated 
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers)); 
    let mut new_numbers: [i32; 3] = [1,2,3];
    println!("Array occupies {} bytes", std::mem::size_of_val(&new_numbers));

    // get slice of an array
    let slice1: &[i32] = &numbers[0..2];
    println!("Slice1: {:?} ", slice1);
    // get another slice 
    let slice2: &[i32] = &numbers[0..3];
    println!("Slice2: {:?} ", slice2);




}