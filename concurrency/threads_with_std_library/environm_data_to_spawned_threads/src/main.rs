use std::thread;


// Accessing Environment data with spawned threads:

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

    // NOTE: this method has some drawbacks thought, if we try to use the variable (the vector 'v' in this example) 
    //       after moving it to the thread we will get an error:   error[E0382]: borrow of moved value: 'v'
    //       because we cannot utilize a variable after it has been moved.
    //       So, am alternative for doing that is using Scoped Threads, see example 'environm_data_with_scoped_threads' 


}