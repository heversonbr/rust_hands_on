#[allow(dead_code)]
pub fn run() {
    // Closures are anonymous functions that can capture variables from their surrounding environment.
    // We say that a closure “close over” values in their scope.
    // They are similar to lambdas or anonymous functions in other languages, but with powerful capabilities around variable capture, ownership, and type inference.
    // You can save a closure in a variable or pass as arguments to other functions,
    // you create the closure in one place and then call the closure to evaluate it in a different context.
    //
    // Sintax:
    // let closure = |arg1, arg2| {
    //      // body
    //      arg1 + arg2
    // };
    // Note: brackets are optional when the closure body has only one expression

    //r
    //
    // Key features:
    // - Type Inference: Rust can ofbnbtiten infer the types of the closure’s parameters and return value.
    // - Variable Capture: Closcbegnures can capture variables from their surrounding scope in three different ways:
    //   -- By reference (&T)
    //   -- By mutable reference (&mut T)
    //   -- By value (T)
    //   Rust decides automatically how to capture based on usage, but you can also force a capture using move.

    // Example 1: Closure Capture. A simple closure that captures a value (by reference) from the scope
    {
        let name = "John";
        // note that, variable name is captured by reference here
        let greeting_closure = || println!("Hello {:?}", name);
        greeting_closure();
    }

    // Example 2: Capture by mutable reference
    {
        let mut x = 5;

        // Captures x by mutable reference
        let mut change_x = || x += 1;
        change_x();
        println!("x is: {}", x); // prints: x is: 6
    }

    // Example 3: We can use 'move' to force the closure to take ownership of captured variables
    {
        let x = String::from("Hello World!");

        let consumer_closure = move || {
            println!(
                "The value [{:?}] is now ownded and consumed by the closure.",
                x
            );
        };

        consumer_closure(); // this will invoke the closure and consume the now owned value 'x'

        // if we try to use x after that, we will get an error, because x does not own the value anymore
        // println!("{}", x); // Error! x has been moved
        // note that, here in this example we did not even passed any parameter to the closure,
        // it captured the values directly from the scope.
    }

    // Unlike regular functions, closures don’t require explicit type annotations for their parameters or return values.
    // Functions often need strict type definitions to ensure consistent usage across different parts of a program.
    // Closures, on the other hand, are typically short-lived and used in limited, local contexts where
    // the compiler can reliably infer the types without needing annotations.
    //
    // Example 4: Fully annotated closure
    {
        let outer_var: i32 = 42; // environment value
        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        println!("closure_annotated: {}", closure_annotated(1));
    }

    // -----------------------------------
    // Passing parameters to closures
    // -----------------------------------
    // Closures capture from the environment to augment their input, not to replace it.
    // Parameters make closures reusable and flexible, while captures make them context-aware.
    // Closures can “remember” their environment — like grabbing local variables — so they can behave like partially applied functions.
    // However, closures still need to operate on fresh, variable input — things that change per call.
    // Captures are fixed at creation time; parameters are dynamic.
    // If we relied only on captures, the closure would be useful once or for only one exact value.
    // So, parameters allow input flexibility at call/invokation time
    //
    // Example 5: creating a closure that receives a parameter 'x' (by reference) and returns a value.
    {
        let add_one_v1 = |x| x + 1; // here we define our closure
        let x = 3; // the value that will be capture by the closure

        println!("closure returns: {:?} ", add_one_v1(x)); // involking the closure that captures 'x'
        println!("x is: {:?} ", x);
        println!("closure returns again: {:?} ", add_one_v1(x));
        // involking the closure again to show that the parameter is passed by reference
    }

    // Example 6: passing the parameter by mutable reference
    //
    // In this example we see a closure called 'square_it' that squares the value it
    // receives as a parameter and at it the 'total' value it captures from the scope.
    //
    {
        let mut total: i32 = 0;

        let mut square_it = |x| {
            total += x * x; // square the mutable value and add to total
            x * x // returns the squared value
        };

        square_it(10); // OK: mutable borrow ends here
        total = -1; // now the mutable variable total can be changed again, because the mutable borrow ended in the last line
                    // note: if we swap the 2 previous lines, the code wont compile because if would break the borrow checker rules
                    // "You can have only one mutable reference to a variable at a time"
                    // As the variable is mutably borrowed, the borrow lasts as long as the closure exists, because the compiler doesn’t know when you might call it again
                    // so if we swap these lines, the code wont compile because we try to access 'total' while it is still borrowed by the closure.
        assert_eq!(-1, total);
    }

    // Example 7: passing the parameter by value, transfer the ownership to the closure
    //
    // If you want to force the closure to take ownership of the values it uses in the environment,
    // you can use the move keyword before the parameter list. This technique is mostly useful when
    // passing a closure to a new thread to move the data so it’s owned by the new thread.
    {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x; // variable moved
                                          // println!("can't use x here: {:?}", x); //  => this print wont compile: value borrowed, after moved before
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }

    // Special Traits:
    //  Closures implement special traits: Fn, FnMut and FnOnce.
    //  These traits are part of the core abstraction Rust uses to model how a closure interacts with captured variables.
    //  Understanding them is crucial when you’re:
    //  - passing closures as arguments to functions (Fn, FnMut and FnOnce): Closures must match how they’re used (read, mutate, consume),
    //  - building iterators or lazy evaluation (Fn): Lazily compute values without mutation or move,
    //  - using closures in UI frameworks (FnMut): handlers my need to mutate state,
    //  - working with concurrency/threads (FnOnce): ownership must be transferred into the thread.
    //  - returning them from functions (all): trait determines how the returned closure can be used,
    //
    // Rust chooses the most restrictive trait necessary:
    // - FnOnce is the most flexible and lowest requirement.
    // - FnMut requires no consumption but allows mutation.
    // - Fn is pure and can be reused, making it safest and most composable.
    //
    // - Trait Fn: typical read-only closures (read a variable but do not change it), capture variables by shared references, it can be re-used.
    //   Common use: callbacks, functional programming.
    //
    // - Trait FnMUt: typical for closures that modify the captured values, capture variables as mutable references.
    //   Commong use: stateful closures, for instance counters.
    //
    // - Trait FnOnce: typical for closures that consume the captured variable, capture variable by value (move), can only be called once,
    //   Common use: One-time usage, when the captured variable is consumed, like spawning threads and ownership transfer.
    //
    // Relationship among these traits
    //
    // Fn automatically implements FnMut and FnMut automatically implements FnOnce.
    // - Fn can be passed to any
    // - FnMut can be passed to FnMut and FnOnce
    // - FnOnce can only be passed to FnOnce
    //

    // Example 8:  Using Fn trait to pass a closure (to a function) that captures a variable by reference

    // here we define a function called call_twice that takes a parameter f,
    // which is a closure (or function) that takes no arguments and returns nothing.
    // The closure type is generic, but it must implement the Fn() trait,
    // meaning it can be called multiple times and doesn’t mutate or consume anything.

    {
        // here we define a function called call_twice that takes a parameter f,
        // which is a closure (or function) that takes no arguments and returns nothing.
        // The closure type is generic, but it must implement the Fn() trait,
        // meaning it can be called multiple times and doesn’t mutate or consume anything.
        use std::ops::Fn;
        fn call_twice<F: Fn()>(f: F) {
            f();
            f();
        }

        let name = "Alice";

        // lets create our closure
        let greeting = || {
            println!("Hello {}", name);
        }; // Closure that implements Fn
           // variable 'name' is capture from the scope by reference

        call_twice(greeting); // when we call the function and pass the closure as a paramenter, this closure must implement Fn()
    }

    // Example 9: Using FnMut to pass a closure (to a function) that captures a mutable variable by reference

    {
        // again, we define the function that will take a closure, called 'f' here, as a parameter.
        // The closure type is generic, but must implement FnMut in this case, in order to allow
        // it to be called mutiple times, and allowing no-concurrent changes
        fn call_twice_mutable<F: FnMut()>(mut f: F) {
            f();
            f();
        }

        let mut counter = 0;
        let increment = || {
            counter += 1; // we increment the captured variable
            println!("Counter: {}", counter);
        };

        call_twice_mutable(increment);

        // Reminder:
        // You only need to declare a closure variable as `mut`
        // if you're calling it directly and it implements `FnMut` or `FnOnce`.
        //
        // If you're passing the closure to a function that takes it as `mut` (e.g., `FnMut`),
        // the mutability is handled *inside* that function — no need to mark the closure as `mut` at the call site.
    }

    // Example 10: Using FnOnce to pass a closure (to a function) that captures by value and consumes this variable.
    {
        // First we define the function that will take the closure as a parameter. we call this parameter 'f' again.
        // The closure type is generic 'F', but it requires the implementation of FnOnce, in order to allow (and restrict) it
        // to be called/invoked only once and consume the captured value when called.
        fn consume_once<F: FnOnce()>(f: F) {
            f();
        }

        // we define the variable that will be captured by the closure
        let message = String::from("Im moving into the closure!");

        // define our closure
        let consumer_closure = move || {
            println!("{:?}", message); // this will consume the captured variable
        };

        // invoking the function and passing the closure into it
        consume_once(consumer_closure);

        // if we uncomment the following line, the code wont compile because it will try to borrow a value that was previously moved
        // println!("{:?}", message);
        // the same happens if we try to call the function with the same closure again  -> consume_once(consumer_closure);
    }

    // TODO: next point to check in details
    // -----------------------------------
    // Returning Closures from Functions
    // -----------------------------------
    //
    // To return a closure from a function, you must use trait objects (Box<dyn Fn...>) or generic trait bounds (for inline closures).}
    // Closures in Rust are anonymous types that capture values from the environment, which means:
    //	•	You can’t write their type directly (it’s complex and compiler-generated).
    //	•	So you need to return them using trait objects or impl Trait.
    // You want to write something like:
    // fn make_adder(x: i32) -> ??? { |y| x + y }
    // But what do you put in place of ???? That’s the key question.
    //
    // Method 1: Using impl Trait (modern and easy):
    //
    //           fn make_adder(x: i32) -> impl Fn(i32) -> i32 {  move |y| x + y }
    //
    //       impl Fn(i32) -> i32 means: “This function returns some type that implements the Fn trait and takes an i32.”
    //       move is required because the closure captures x by value, and it needs to live independently after the function returns.
    //       Without move, the closure would borrow x, but x is local to make_adder() and would be dropped — leading to a dangling reference. So move makes the closure own its captures.
    //
    // Method 2: Using trait objects (Box<dyn Fn()>)
    //           useful when: You don’t know the concrete closure type or You need dynamic dispatch (e.g., heterogeneous returns, trait objects)
    //
    //             fn make_logger(message: String) -> Box<dyn Fn()> { Box::new(move || println!("{}", message)) }
    //
    //        Box<dyn Fn()> means: “This function returns a heap-allocated closure implementing Fn().”
    //        We box the closure because trait objects need to be behind a pointer (heap or reference).
    //        We use move to capture message by value so the closure owns it.
    //
    // ----------------------------------------------------------------------------------------------
    //  Summary:
    // ----------------------------------------------------------------------------------------------
    //  Approach                 Return Type                  Use When…
    //  impl Fn     i          impl Fn(...) -> ...      Closure type is known and consistent
    // Trait object (boxed)    Box<dyn Fn...>           Need dynamic dispatch, heterogeneous returns
    // ----------------------------------------------------------------------------------------------
}
