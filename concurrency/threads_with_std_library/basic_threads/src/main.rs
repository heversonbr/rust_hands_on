use std::thread;
use std::time::Duration;

// Creating a New Thread with spawn
// To create a new thread, we call the thread::spawn function, 
// and pass a closure to it, this closure contains the code we want to run in the new thread.


fn main(){

    // Creating 20 threads that will print some text at each time interval 
    thread::spawn( || {
        for i in 1..20 {
            println!("[SPAWNED THREAD] : Message from thread #:{} ", i);
            thread::sleep(Duration::from_millis(10));
        }
    });


    for i in 1..10 {
        println!("[MAIN THREAD] : Message #{} from the MAIN thread!", i);
        thread::sleep(Duration::from_millis(10));
    }

    // Note that when the main thread of a Rust program completes, 
    // all spawned threads are shutdown, whether or not they have finished running. 
    // The output from this program might be a little different every time, 
    // And even though we told the spawned thread to print until 'i' is 20, 
    // it only got to 9 before the 'main' thread shut down.
    // Therefore, the spawned thread may end prematurely or not run at all. 

    // See the example 'joining threads' in order to see how to fix this behavior

}

