use std::thread;


// Scope in rust: 
// A scope is a chunk of code that is contained within a code block. 
// The Rust compiler restricts access to variables and constants 
// inside a scope to lines and other scopes within that scope.

// SCOPED THREADS allow you to borrow variables from another scope without moving them. 
// Unscoped threads need you to move the variable to the thread you want to utilize it in, 
// which stops it from being used in its previous scope.
// Remember: You can borrow an immutable reference to as many threads as you like in scoped threads, 
//           but you can only borrow a mutable reference once.

fn main() {

    println!("Example 1:");
    // 1) Threads using external variables from their scopes 
    let v = vec![ 1,2,3,4,5,6 ];
    let midpoint = v.len() / 2;

    // Create a scope 
    std::thread::scope(|scope| {
        // spawn first scoped thread
        scope.spawn(|| {
            let first = &v[..midpoint];
            println!("[scoped thread 1] : first half of v -> {first:?}");
        });
        // spawn second scoped thread
        scope.spawn(|| {
            let second = &v[midpoint..];
            println!("[scoped thread 2] : second half of v -> {second:?}");
        });

    });
     // all threads within the scope have to be closed, for the program to continue

    // Main thread: here we can use 'v' again because with scoped thread we dont need to move the variable ownership to the threads
    println!("Main thread full v: {v:?}");


    // 2) Now suppose we want to access returning values from threads
    println!("Example 2:");

    let words = "Hello, world";

    let (t1, t2) = thread::scope(|scope| {
    // spawn first thread
    let t1 = scope.spawn(|| {
      format!("{} from thread 1", words)
     });
  
     // spawn second thread
      let t2 = scope.spawn(|| {
      format!("{} from thread 2", words)
     });

      // get results of both threads and return
      return (t1.join().unwrap(), t2.join().unwrap());
    });

    println!("thread 1: {}\nthread 2: {}", t1, t2);

}
