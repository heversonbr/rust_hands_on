mod selection_sort;
///  SELECTION SORT:
/// 1) Initialize minimum value(min_idx) to location 0.
/// 2) Traverse the array to find the minimum element in the array.
/// 3) While traversing if any element smaller than min_idx is found then swap both the values.
/// 4) Then, increment min_idx to point to the next element.
/// 5) Repeat until the array is sorted.
/// 

fn main() {


    // declare a mutable variable  that points to an array or a vector of integers
    //let mut my_array: [i32; 6] = [33,12,45,7,4,21];
    //let mut my_array: Vec<i32> = vec![10,33,12,56,45,100,7,4,21];
    //let mut my_array: Vec<i32> = vec![3,2];
    let mut my_array: Vec<i32> = vec![1];
    //let mut my_array: Vec<i32> = vec![];

    println!("Start array {:?}", my_array);
    // passes a mutable reference to a variable
    
    // SELECTION SORT WITHOUT RECURSION
    selection_sort::regular_selection_sort(&mut my_array);
    println!("(REGULAR SELECTION SORT) Resulting array {:?}", my_array);

    // SELECTION SORT WITH RECURSION
    //let size = my_array.len();
    //selection_sort::recursion_selection_sort(&mut my_array, 0, size);
    //println!("(RECURSION SELECTION SORT) Resulting array {:?}", my_array);

}
