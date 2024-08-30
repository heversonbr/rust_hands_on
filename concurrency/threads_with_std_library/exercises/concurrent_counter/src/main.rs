use std::sync::{Arc, Mutex};
use::std::thread;

fn main() {
    
    // My counter must a value with shared ownnership among all the threads ->  counter: Arc<i32>; 
    // so I could declare my counter like this:   let mut counter: Arc<i32> = Arc::new(0);
    // however, besides the shared ownnership, I want  to ensure that only one thread increments the counter at a time
    // therefore, I need wrap my counter with a Mutex<T> in order to provides exclusive, mutable access to the data.
    // The Mutex guarantees that only one thread can lock the mutex at a time, 
    // ensuring that only one thread can access or modify the data.
    // Thus, I declare my counter like this: 
    
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    // We use an Arc to share memory among threads, and the data inside the Arc is protected with a mutex.

    let mut handles = vec![];

    // Spawn 10 threads
    for t in 1..=10 {
        // a shared reference created for each thread
        let counter_shared_ref = Arc::clone(&counter);    

        let handle = thread::spawn( move || { 
            // lock() : Acquires a mutex, blocking the current thread until it is able to do so.
            let mut counter_value = counter_shared_ref.lock().unwrap();
            // update the value
            println!("Thread {} reading counter: {}", t, *counter_value);
            *counter_value += 1;
            println!("Thread {} updated counter: {}", t, *counter_value);

        });
        // add the handle to a vector of handles 
        handles.push(handle);
    }

    // After spawning the threads, the main thread waits for each of them to finish 
    // by calling join on each handle stored in the handles
    for handle in handles {
        handle.join().expect("Error joinning threads!");
    }

    // as our counter is a mutex, we lock() it in order to read its value.
    println!("Counter is : {:?}", counter.lock().unwrap());
    

    // Some notes: 
    // To learn when a thread completes, it is necessary to capture the JoinHandle object that is returned by the call to spawn, 
    // which provides a join method that allows the caller to wait for the completion of the spawned thread. 
    // spawn() : Spawns a new thread, returning a JoinHandle for it.
    // The join method returns a thread::Result containing Ok of the final value produced by the spawned thread, 
    // or Err of the value given to a call to panic! if the thread panicked.


}
