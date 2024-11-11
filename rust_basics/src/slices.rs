// Slices provide a view into a sequence of elements, usually part of an array, String or vector:
// Array/Vector slices: Non-owning reference to a portion of an array or a vector
// Sintax: &[T]
// String slices: Non-owning reference to part of a String or string literal.
// Sintax: &str

{   // example of an array slice 
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4]; // Refers to elements [2, 3, 4]
}

{   // example of a String slice
    let s = String::from("hello");
    let slice: &str = &s[1..4]; // Refers to "ell"
}

{
    // example of a Vector slice
    let vec = vec![10, 20, 30, 40];
    let vec_slice: &[i32] = &vec[1..3];    // Slices the vector, gives &[20, 30]
}

// Rust has a concept called deref coercion, where certain types can be automatically coerced into slices or other references when needed.
// Arrays ([T; N]) can be coerced into slices (&[T])
// A String can be coerced into &str because String is essentially a growable heap-allocated array of UTF-8 bytes, and a string slice (&str) is a reference to part of a String
// A Vec<T> can be coerced into a slice (&[T]), similar to arrays


//  slices are a specific type of reference in Rust.
// They reference a portion of data rather than owning it. 
// Slices are non-owning and borrow data. This means:
// - A slice is tied to the lifetime of the original data.
// - Slices can't outlive the data they point to, ensuring safety through Rust’s borrowing rules.

