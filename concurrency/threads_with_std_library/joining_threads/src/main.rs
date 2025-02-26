use std::thread;
use std::time::Duration;

// NOTE: this example 'joining_threads' is connected to the 'basic_threads' example in the same directory

// In the example 'basic_threads', the code not only stops the spawned thread prematurely 
// most of the time, due to the main thread ending, but also can not guarantee that 
// the spawned thread will run at all, 
// because there is no guarantee on the order in which threads run.

fn main() {

    // The return type of thread::spawn is JoinHandle.
    // A JoinHandle is an owned value that, when we call the join method on it, it will wait for its thread to finish. 
    // Here we show how to use the JoinHandle of the thread and call join to make sure the spawned thread finishes before main exits.
    println!("-> Create and spawn 20 threads that should print some text every 10 mili seconds!");
    println!("   Calling join will block the currently running thread until the thread terminates.\n");

    // thread::spawn returns a JoinHandle 
   let handle = thread::spawn(|| {
       for i in 1..20 {
           println!("hi number {} from the [SPAWNED] thread!", i);
           thread::sleep(Duration::from_millis(10));
       }
   });

   println!("-> Main thread will print 10 times a message every 10 miliseconds!\n");
   for i in 1..10 {
       println!("hi number {} from the [MAIN] thread!", i);
       thread::sleep(Duration::from_millis(10));
   }

   
   // The JoinHandle has a .join() method that blocks.
   // Use let handle = thread::spawn(...) and later handle.join() to wait for the thread to finish and have the program count all the way to 10.
   // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. 
   // Blocking a thread means that thread is prevented from performing work or exiting.
   // The two threads will alternate, but the main thread waits and does not end until the spawned thread is finished.
   // In this example all the spawned threads will be executed , different from the previous example in 'basic_threads'.
   handle.join().unwrap();
   println!("NOTE: Calling join on the handle, blocks the thread currently running until the thread represented by the handle terminates!\n      All the threads will run until the end now.");
   // Small details, such as where join is called, can affect whether or not your threads run at the same time
   // Check what happens if we  move handle.join() before the for loop in main. 
   // the executiion will wait for all the spawned threads to finish running before print the message from the main thread, 


}
