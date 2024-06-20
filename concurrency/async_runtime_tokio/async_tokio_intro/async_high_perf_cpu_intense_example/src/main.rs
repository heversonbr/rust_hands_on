use tokio::{io::AsyncReadExt, time};

// #4 : The forth example adds some CPU intense calculation to the asyncronous execution. But uses spawned threads in order to avoid depreciation on the performance 
//      as discussed in the example #3. 

// the new reader implementation for the cpu intense reader using spawned threads.
fn fib(n: u32) -> u32 {
    match n { 
        0 => 0,
        1 => 1,
        n => fib(n-1) + fib(n-2), 
    }
}

async fn async_sleeper(){
    log::info!("[ASYNCRONOUS] Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("[ASYNCRONOUS] Awake");
}


async fn cpu_intense_async_reader_high_performance(){
    log::info!("[ASYNCRONOUS SPAWNED THREAD] Reading some data...");
    let mut f = tokio::fs::File::open("../data.csv").await.expect("Error opening file!");
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
    
    // Run CPU intense asyncronous tasks example 
    // (with GOOD performance -> spawn a new thread for intense CPU operations): 
    // create an instance of the tokio run time
    let rt = tokio::runtime::Runtime::new().unwrap();
    // we call our run funtion which returns a Future.
    let future = run_asyncronously_cpu_intense_tasks_fixed_performance(); 
    // we pass the future to the block_on function of the runtime
    let start =  std::time::Instant::now();
    rt.block_on(future);
    let end =  std::time::Instant::now();
    log::info!("[CPU INTENSE ASYNCRONOUS - SPAWNED THREADS - FIXED PERFORMANCE] TOOK: {:?} seconds to run ", end-start);
 

}
