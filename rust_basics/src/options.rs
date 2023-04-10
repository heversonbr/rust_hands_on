
#[allow(dead_code)]
pub fn run(){

    // In Rust we can access an Option value using pattern matching 
    // or the unwrap method.  see the examples: 
    println!("Examples of accessing Option value:");
    println!("-----------------------------------");
    
    // 1) using pattern matching
    println!("1> using pattern matching:");
    let y: Option<i32> = Some(15);
    println!("The Option is {:?}", y);
    match y {
        Some(extracted_value) => {  
            println!("The value is {:?}",extracted_value); 
        },
        None => { 
            println!("The option is empty");
        }
    }

    println!("-----------------------------------");
    // 2) using unwrap: 
    println!("2> using unwrap:");
    let x: Option<i32> = Some(5);
    println!("The Option is {:?}", x); 
    println!("The value is {:?}", x.unwrap()); 
    // NOTE: accessing the value directly with unwrap is only recommended 
    //       if you're certain the option contains a value
    // ------------------------------------------------------------------
    // However, these methods MOVE the value of a variable to another varible. 
    // It changes the ownership.
    // ------------------------------------------------------------------
    // In order to access the value of an Option without MOVING the value,
    // you can access the value of an Option without moving it using 
    // the 'as_ref' method or the 'as_mut' method if you need mutable access.
    // - as_ref : returns an Option containing a reference to the value inside 
    // the Option if Option is Some, or None if the Option is None. Example:
   
    println!("-----------------------------------");
    // 3) using as_ref() 
    println!("3> using as_ref():");
    let z: Option<i32> = Some(25);
    println!("The Option is {:?}", z); 

    let extracted_ref = z.as_ref(); 
    match extracted_ref {
        Some(ref_to_value) => {
            println!("The value is {:?}", ref_to_value);  
        },
        None => {
            println!("The option is empty");
        }
    }

    println!("-----------------------------------");
    // using as_mut
    // - as_mut : Similarly, we can use this method to get a mutable reference 
    // to the value inside the Option. 
    println!("4> using as_mut():");
    let mut w: Option<i32> = Some(25);
    println!("The Option is {:?}", w); 
    let extracted_mut_ref = w.as_mut(); 
    match extracted_mut_ref {
        Some(mut_ref_to_value) => {
            println!("The value is {:?}", mut_ref_to_value);  
            *mut_ref_to_value += 5; 
            println!("Changed value is {:?}", mut_ref_to_value);
        },
        None => {
            println!("The option is empty");
        }
    }

    println!("-----------------------------------");
    // example using an immutable boxed value: 
    // returns an Option containing a reference to the Box inside the Option 
    // if Option is Some, or None if the Option is None.
    println!("5> using as_ref() with an immutable boxed value:");
    let boxed_option : Option<Box<i32>> = Some(Box::new(10));
    println!("Boxed Option: {:?}", boxed_option);
    match boxed_option.as_ref() {
        Some(ref_to_box) => {
            println!("Box Value: {:?}", &ref_to_box);
        }
        None => {
            println!("The Option is empty");
        }
    }
    println!("Boxed Option: {:?}", boxed_option);

    println!("-----------------------------------");
    // example using an mutable boxed value: 
    println!("6> using as_mut() with a utable boxed value:");
    let mut boxed_option : Option<Box<i32>> = Some(Box::new(20));
    println!("Mut Boxed Option: {:?}", boxed_option);
    match boxed_option.as_mut() {
        Some(mut_ref_to_box) => {
            println!("Box Value: {:?}", mut_ref_to_box);
            **mut_ref_to_box += 15;
            println!("Changed Box Value: {:?}", mut_ref_to_box);
        }
        None => {
            println!("The Option is empty");
        }
    }
    println!("Mut Boxed Option: {:?}", boxed_option);


}