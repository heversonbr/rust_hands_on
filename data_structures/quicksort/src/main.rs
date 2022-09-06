mod quicksort;

fn main() {
    
    // declare an array of integers 
    let mut my_array: [i32;9] = [33, 35, 22, 42, 11, 17, 8, 13, 27];
    //let mut my_array: [i32; 6] = [33,12,45,7,4,21];
    //let mut my_array: Vec<i32> = vec![10,33,12,56,45,100,7,4,21];
    //let mut my_array: Vec<i32> = vec![3,2,1];
    //let mut my_array: Vec<i32> = vec![0];

    println!("Input Array : {:?} ", my_array);
    let size = my_array.len();
    println!("size: {:?}" , size);

    quicksort::quicksort(&mut my_array, 0 , size); 
    println!("Sorted Array : {:?} ", my_array);

}
