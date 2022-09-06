// Pointer and references
// Reference pointers point to a resource in memory 
#[allow(dead_code)]

pub fn run(){


    // Primity array:
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("Array Values: {:?} ", (arr1, arr2));

    // With mon primity data (ex: vectors), if you assign another variable to a place of data,
    // the first variable will no longer hold that value. You will need to use a reference to point 
    // to the resource. 

    // Ex: vector as it is not a primitive value
    let vec1 = vec![1,2,3];
    // using list this => let vec2 = vec1;  way will catch an error, 
    // we should use the 'ampersand' symbol '&' in this case
    let vec2 = &vec1;
    println!("Vec Values: {:?} ", (&vec1, vec2));


}

