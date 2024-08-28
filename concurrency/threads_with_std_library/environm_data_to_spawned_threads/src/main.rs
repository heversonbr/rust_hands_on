use std::thread;

// In this example we discuss/show how to allow spawned threads to access data from the current environment.
// This example also shows how to use move, in order to give ownership of values to a thread.
// We'll often use the 'move' keyword with closures passed to 'thread::spawn'
// because the 'closure' will then take ownership of the values it uses 'from the environment', 
// Thus, transferring ownership of those values from one thread (the main thread in this case) to another(the new spawned thread).
// Therefore, to use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it need.

// By adding the 'move' keyword before the closure, we force the closure to take ownership of the values it’s using 
//                        rather than allowing Rust to infer that it should borrow the values.


fn main() {

    println!("In this example we see how to allow spawned threads to access data from the current environment.");
    // the vector 'v' declared in the main thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn( move || {

        println!("Here's a vector in a spawned thread: {:?}", v);
        println!("The spawned thread took ownership of the vector declared in the environment")
    });


    handle.join().unwrap();



}