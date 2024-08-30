use std::{sync::{Arc, RwLock}, thread};
// we can use the rand crate to generate random values to be added by writer threads.
// use rand::Rng;


fn main(){

    // My vector of numbers (i've chosen i32 as type, could be something else)
    // let shared_vec:Vec<i32> = Vec::new();
    // but, my vector must be shared, and it is a multi-threaded context, 
    // therefore I will need an Arc<T> wrapping it, then:
    // let shared_vec: Arc<Vec<i32>> = Arc::new(Vec::new());
    // moreover, I want to have multiple threads to read and write to these value simultaneously, 
    // then I need to wrap my value in a RwLock<T> smart pointer, because it is the 
    // thread-safe read-write lock allowing multiple readers or one writer, finally my vector is:
    let shared_vec: Arc<RwLock<Vec<i32>>> = Arc::new(RwLock::new(Vec::new()));

    // In order to make the main thread to wait all the other threads to finish before finishing the main thread
    // we use the JoinHandle object returned by the spawn methods. We use this vector of join handlers in order to call
    // the join method, to make the main thread to wait. 
    let mut handles = vec![];

    // Spawn 5 reader threads
    for r in 1..=5  {
        let shared_vec_ref = Arc::clone(&shared_vec);
        let handle = thread::spawn( move || {
            let current_vec = shared_vec_ref.read().unwrap();
            println!("Reader {} sees Vec: {:?}" , r, *current_vec); 

        });
        handles.push(handle);
    }

    // Spawn 2 writer threads
    for w in 1..=2 {
        let shared_vec_ref = Arc::clone(&shared_vec);
        // Create a random number generator and Generate a random number between 10 and 50
        //let random_number = rand::thread_rng().gen_range(1..10);

        let handle = thread::spawn(move || {
            let mut current_vec = shared_vec_ref.write().unwrap();
            println!("Writer {} sees current Vec: {:?}" , w, current_vec); 
            //current_vec.push(random_number);
            current_vec.push(w);
            println!("Writer {} wrote in current Vec: {:?}" , w, *current_vec); 

        });
        handles.push(handle);
    }

    // Next we join all the JoinHandle objects to make the main thread to wait
    for handle in handles{
        handle.join().expect("Error joinning threads!");
        // join returns a Result. we use expect here to catch the error if it panics, 
        // it is better to handle this error gracefully , by using for instance a match pattern.
    }

    println!("Main Thread sees SHARED VEC: {:?}" , shared_vec);

}
