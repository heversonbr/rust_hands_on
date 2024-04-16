use tokio::{io::AsyncReadExt, runtime, time};
use std::{arch::x86_64::_MM_FROUND_CUR_DIRECTION, io::Read};

// We have a program that does 2 main things:  
//      1) Sleeps for 2 seconds, 
//      2) reads some random data from a file 
//
// This exercice is divided in 3 different examples:
// --------------------------------------------------------
// #1 : The first example runs the program syncronously. 
// Syncronous execution :  
fn sync_sleeper(){
    log::info!("[SYNCRONOUS] Sleeping");
    std::thread::sleep(std::time::Duration::from_secs(1));
    log::info!("[SYNCRONOUS] Awake");
}

fn sync_reader(){
    log::info!("[SYNCRONOUS] Reading some data...");
    let mut f1= std::fs::File::open("data.csv").unwrap();
    let mut contents = String::new();
    f1.read_to_string(&mut contents).unwrap();
    log::info!("[SYNCRONOUS] Just read {} bytes from file" , contents.len());
}
// this function will run syncronously 
// the sleeper will only sleep after all reading happens.
fn run_syncronously(){

    let start =  std::time::Instant::now();
    for _ in 0..10 { 
        sync_reader();
    }
    sync_sleeper();
    let end =  std::time::Instant::now();
    log::info!("[SYNCRONOUS] TOOK: {:?} seconds to run ", end-start);
}

// --------------------------------------------------------
// #2 : The second example runs the program asyncronously. 
// Asyncronous execution :  
async fn async_sleeper(){
    log::info!("[ASYNCRONOUS] Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("[ASYNCRONOUS] Awake");
}

async fn async_reader(){
    log::info!("[ASYNCRONOUS] Reading some data...");
    let mut f = tokio::fs::File::open("data.csv").await.expect("Error opening file!");
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.expect("Error reading file");
    log::info!("[ASYNCRONOUS] Just read {} bytes from file" , contents.len());
}

async fn run_asyncronously() {
    tokio::join!(
        async_sleeper(),
        async_reader(),
        async_reader(),
        async_reader(),
        async_reader(),
        async_reader(),
        async_reader(),
        async_reader(),
        async_reader(),
        async_reader(),
        async_reader(),
    );
}
// --------------------------------------------------------
// #3 : The thid example adds some CPU intense calculation to the asyncronous execution.
//      This is not good for tokio as it uses a single thread for its main event loop, therefore any task performing cpu intense operation slows down the other asyncronous tasks. 
//      Asyncronous execution with CPU intence operation:  
//      Lets add a fibonacci calculation for each reader calls of the previous example. 
fn fib(n: u32) -> u32 {
    match n { 
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2), 
    }
}
// the new reader implementation for the cpu intense reader
async fn cpu_intense_async_reader(){
    log::info!("[CPU INTENSE ASYNCRONOUS - LOW PERF] Reading some data...");
    let mut f = tokio::fs::File::open("data.csv").await.expect("Error opening file!");
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.expect("Error reading file");
    log::info!("[CPU INTENSE ASYNCRONOUS - LOW PERF] Just read {} bytes from file" , contents.len());
    // just adds the fibonacci calculation for each time it reads data
    log::info!("[CPU INTENSE ASYNCRONOUS - LOW PERF] Computing Fib(40)");
        fib(40);
        log::info!("[CPU INTENSE ASYNCRONOUS - LOW PERF] Done Computing Fib(40)");

}

async fn run_asyncronously_cpu_intense_tasks() {
    tokio::join!(
        async_sleeper(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
        cpu_intense_async_reader(),
    );
}


// --------------------------------------------------------
// #4 : The forth example adds some CPU intense calculation to the asyncronous execution. But uses spawned threads in order to avoid depreciation on the performance 
//      as discussed in the example #3. 

// the new reader implementation for the cpu intense reader using spawned threads.
async fn cpu_intense_async_reader_high_performance(){
    log::info!("[ASYNCRONOUS SPAWNED THREAD] Reading some data...");
    let mut f = tokio::fs::File::open("data.csv").await.expect("Error opening file!");
    let mut contents = vec![];
    f.read_to_end(&mut contents).await.expect("Error reading file");
    log::info!("[ASYNCRONOUS SPAWNED THREAD] Just read {} bytes from file" , contents.len());

    // creates a new thread for each CPU intense operations.
    tokio::task::spawn_blocking(move || {
        log::info!("[ASYNCRONOUS SPAWNED THREAD] Computing Fib(40)");
        fib(40);
        log::info!("[ASYNCRONOUS SPAWNED THREAD] Done Computing Fib(40)");
    }
    ).await.unwrap();
    
}

async fn run_asyncronously_cpu_intense_tasks_fixed_performance() {
    tokio::join!(
        async_sleeper(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
        cpu_intense_async_reader_high_performance(),
    );
}

fn main() {

    // create the simple logger:
    simple_logger::init_with_level(log::Level::Info).expect("Error creating logger.");
    
    {   // Run syncronous example: 
        run_syncronously();
        
    }

    println!("---------------------------------------");
    {   // Run asyncronous example: 
   //     // create an instance of the tokio run time
        let rt = tokio::runtime::Runtime::new().unwrap();
        // we call our run funtion which returns a Future.
        let future = run_asyncronously(); 
        // we pass the future to the block_on function of the runtime
        let start =  std::time::Instant::now();
        rt.block_on(future);
        let end =  std::time::Instant::now();
        log::info!("[ASYNCRONOUS] TOOK: {:?} seconds to run ", end-start);
    }

    println!("---------------------------------------");
    {   // Run CPU intense asyncronous tasks example (low performance verification): 
   //     // create an instance of the tokio run time
        let rt = tokio::runtime::Runtime::new().unwrap();
        // we call our run funtion which returns a Future.
        let future = run_asyncronously_cpu_intense_tasks(); 
        // we pass the future to the block_on function of the runtime
        let start =  std::time::Instant::now();
        rt.block_on(future);
        let end =  std::time::Instant::now();
        log::info!("[CPU INTENSE ASYNCRONOUS - LOW PERF] TOOK: {:?} seconds to run ", end-start);
    }


    println!("---------------------------------------");
    {   // Run CPU intense asyncronous tasks example 
        // (with GOOD performance -> spawn a new thread for intense CPU operations): 
   //     // create an instance of the tokio run time
        let rt = tokio::runtime::Runtime::new().unwrap();
        // we call our run funtion which returns a Future.
        let future = run_asyncronously_cpu_intense_tasks_fixed_performance(); 
        // we pass the future to the block_on function of the runtime
        let start =  std::time::Instant::now();
        rt.block_on(future);
        let end =  std::time::Instant::now();
        log::info!("[CPU INTENSE ASYNCRONOUS - SPAWNED THREADS - FIXED PERFORMANCE] TOOK: {:?} seconds to run ", end-start);
    }




    // Last but not the least!!!
    //
    // We can actually use the tokyo main macho in order to make it easier to use the tokyo runtime
    // if we use #[tokyo::main] macro before the main function we dont need to create an instance the runtime, the macro will do for us
    // this will turn the main function into an async function and we can just call our 'run' async functions from our main, 
    // for in order to run our previous examples we just need to do the following:

    // #[tokyo::main]
    // async fn main(){
    //     run_asyncronously().await; 
    // }

    


}
