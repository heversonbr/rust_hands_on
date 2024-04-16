use std::thread;

// We'll often use the move keyword with closures passed to 'thread::spawn'
// because the closure will then take ownership of the values it uses from the environment, 
// thus transferring ownership of those values from one thread to another.
// To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it need.

// By adding the move keyword before the closure, we force the closure to take ownership of the values 
//       it’s using rather than allowing Rust to infer that it should borrow the values.


fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}