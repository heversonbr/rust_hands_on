// In this file we will discuss data types in rust
// NOTE1: Rust is a statically typed language. it means that we need to know the type of all values at the compiling time.
//        The compiler can infer the type based on the value , as shown later. 
// Ref for types: https://doc.rust-lang.org/reference/types.html
#[allow(dead_code)]

pub fn run(){

    // Scalar Types:
    // - Numeric:
    // -- signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    // -- unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    // -- floating point: f32, f64
    //    (Integers default to i32 and floats to f64)
    // - Textual
    // - char: Unicode scalar values like 'a', 'α' and '∞' (4 bytes each). 
    //   NOTE: In Unicode, the code points from U+D800 to U+DFFF are called surrogates. 
    //         They are reserved for use by UTF-16, and you're not allowed to use them for anything else.
    //         Unicode Scalar Values are represented as a 32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF range. 
    //         It is immediate Undefined Behavior to create a char that falls outside this range.
    // - str: is represented the same way as [u8], it is a slice of 8-bit unsigned bytes.
    //        Since str is a dynamically sized type, it can only be instantiated through a pointer type, such as &str.
    // - Boolean
    // -- bool: either true or false


    //Compound Types:
    // - arrays: like [1, 2, 3]   , note: arrays are fixes length
    //   An array is a fixed-size sequence of N elements of type T. The array type is written as [T; N].
    // -- A slice is a dynamically sized type representing a 'view' into a sequence of elements of type T. 
    //    The slice type is written as [T].
    //    Slice types are generally used through pointer types.
    // - tuples: like (1, true)
    //   Tuple types are a family of structural types1 for heterogeneous lists of other types.
    //   -- unit type (): whose only possible value is an empty tuple: ()
    //      For convenience and historical reasons, the tuple type with no fields (()) is often called unit or the unit type. 
    //      Its one value is also called unit or the unit value.
    

    // NOTE3: in this example variables have an underscore '_' before their names 
    //        to avoid the warning =>  ^^^^ help: if this is intentional, prefix it with an underscore: `_mutable`
    
    
    // 1) Variables can be type annotated.
    let _logical: bool = true;
    let _an_integer   = 5i32; // Suffix annotation => i32
    let _a_float: f64 = 1.0;  // Regular annotation => f64
    let _is_greater: bool = 10 < 5;  // is_greater will receive false.

    // Or a default will be used.
    let _default_integer = 7;   // default `i32`
    let _default_float   = 3.0; // default `f64`
    println!("Max i32 is {} ", std::i32::MAX);
    println!("Max i64 is {} ", std::i64::MAX);
    
    
    // 2) A type can also be inferred from context 
    let mut _inferred_type = 12; // Type i64 is inferred from another line, see next line.
    _inferred_type = 4294967296i64;


    // 3) A mutable variable's value can be changed.
    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;
    println!("{}", _mutable);
    
    // 5) Error! The type of a variable can't be changed.
    //    ex, attributing this:  'mutable = true;' when  the variable was previously declared   'let mut _mutable = 12;' 
    //    must generate this error: 'expected integer, found `bool`'
    
    // 6) Variables can be overwritten with shadowing. 
    {
        let _mutable = true;
        println!("{}", _mutable);
    }
    println!("{}", _mutable);
    // NOTE:    Variable shadowing occurs when a variable declared within a certain scope (decision block, method, or inner class) has the 
    //          same name as a variable declared in an outer scope. At the level of identifiers (names, rather than variables), this is known 
    //          as name masking. This outer variable is said to be shadowed by the inner variable, while the inner identifier is said to mask 
    //          the outer identifier. This can lead to confusion, as it may be unclear which variable subsequent uses of the shadowed variable 
    //          name refer to, which depends on the name resolution rules of the language. Ref: https://en.wikipedia.org/wiki/Variable_shadowing

    // 7) Char

    let a1 = 'a';
    // let b1 = 'ab'  will give an error!!!! 
    let c1 = '\u{1F600}';     // can be an unicode  \u{<unicode_code>}
    println!("This is char a1: {} and this is char b1: {} using unicode!" , a1, c1);


    // 8) A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type 
    // signature (T1, T2, ...), where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any 
    // number of values.
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    // Tuples are printable
    println!("long_tuple with different types: {:?}", long_tuple);
    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple first value: {}", long_tuple.3);
    // Tuples can be tuple members

    // But long Tuples cannot be printed (max 12 elements?):  Uncomment the 2 lines to see the compiler error
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);


    // 9) 
    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 10) Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    

}