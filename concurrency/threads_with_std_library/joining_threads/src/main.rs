use std::thread;
use std::time::Duration;

// In the previous example 'basic_threads' , the code not only stops the spawned thread prematurely most of the time due to the main thread ending, 
// but because there is no guarantee on the order in which threads run, we also can’t guarantee that the spawned thread will get to run at all!

fn main() {

    // The return type of thread::spawn is JoinHandle.
   // A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish. 
   // Here we show how to use the JoinHandle of the thread and call join to make sure the spawned thread finishes before main exits.

   let handle = thread::spawn(|| {
       for i in 1..10 {
           println!("hi number {} from the spawned thread!", i);
           thread::sleep(Duration::from_millis(1));
       }
   });

   
   for i in 1..5 {
       println!("hi number {} from the main thread!", i);
       thread::sleep(Duration::from_millis(1));
   }

   // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. 
   // Blocking a thread means that thread is prevented from performing work or exiting.
   // The two threads will alternate, but the main thread waits and does not end until the spawned thread is finished.
   // In this example all the spawned threads will be executed , different from the previous example in 'basic_threads'.
   handle.join().unwrap();

   // Small details, such as where join is called, can affect whether or not your threads run at the same time
   // Check what happens if we  move handle.join() before the for loop in main. 
   // the executiion will wait for all the spawned threads to finish running before print the message from the main thread, 


}
