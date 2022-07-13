// Vectors: resizable arrays.



pub fn run(){

    // declare a vector: 
    let mut numbers: Vec<i32>  = vec![1,2,3,4,5];

    println!("{:?}", numbers);

    // get a single value
    println!("Get and Print a single value of a vector: {}", numbers[0]);

    // get array length 
    println!("Get Vector length: {}", numbers.len());

    // reassigning values to the vector
    numbers[2] = 20;
    println!("{:?}", numbers);


    // Add on to Vector
    numbers.push(6);
    numbers.push(7);
    println!("{:?}", numbers);
    println!("Get Vector length: {}", numbers.len());


    // arrays are stack allocated 
    println!("Vector_1 occupies {} bytes", std::mem::size_of_val(&numbers));
    let other_numbers: Vec<i32>  = vec![1];
    println!("Vector_2 occupies {} bytes", std::mem::size_of_val(&other_numbers));


    // get slice of an array
    let slice1: &[i32] = &numbers[0..2];
    println!("Vector Slice 1: {:?} ", slice1);
    // another slice
    let slice2: &[i32] = &numbers[0..3];
    println!("Vector Slice 2: {:?} ", slice2);


    // Loop through vector items
    for x in numbers.iter(){
        println!("Number {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }
    println!("Vector:  {:?}", numbers);




}