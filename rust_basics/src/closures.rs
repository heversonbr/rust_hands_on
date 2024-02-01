pub fn test_closures(){


    {
        let add_one_v1 = |x| { x + 1 } ;  
        // NOTE: we can also define this closure as:  |x| x + 1;  
        //       because brackets are optional when the closure body has only one expression
        let x = 3;
        println!("result : {:?} ", add_one_v1(x));
    }
    
    // Closures don’t require you to annotate the types of the parameters or the return value like fn functions do.
    // Defining this interface rigidly is important in Functions for ensuring that everyone agrees on what types of values a function uses and returns.
    // Closures are usually short and relevant only within a narrow context, within these limited contexts, the COMPILER IS RELIABLY ABLE to infer the types. 
    {   
        // Example: fully annotated closure definition 
        let outer_var = 42;
        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        println!("closure_annotated: {}", closure_annotated(1));
    }

    {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;   // variable moved due to use in closure
        // println!("can't use x here: {:?}", x);   => this print wont compile: value borrowed here after moved before 
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));

        // If you want to force the closure to take ownership of the values it uses in the environment, 
        // you can use the move keyword before the parameter list. This technique is mostly useful when 
        // passing a closure to a new thread to move the data so it’s owned by the new thread.

    }
}