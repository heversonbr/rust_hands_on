use tokio::{io::AsyncReadExt, time};

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

async fn async_sleeper(){
    log::info!("[ASYNCRONOUS] Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("[ASYNCRONOUS] Awake");
}

// the new reader implementation for the cpu intense reader
async fn cpu_intense_async_reader(){
    log::info!("[CPU INTENSE ASYNCRONOUS - LOW PERF] Reading some data...");
    let mut f = tokio::fs::File::open("../data.csv").await.expect("Error opening file!");
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


fn main() {
    

     // create the simple logger:
     simple_logger::init_with_level(log::Level::Info).expect("Error creating logger.");
    
    // Run CPU intense asyncronous tasks example (low performance verification): 
    // create an instance of the tokio run time
    let rt = tokio::runtime::Runtime::new().unwrap();
    // we call our run funtion which returns a Future.
    let future = run_asyncronously_cpu_intense_tasks(); 
    // we pass the future to the block_on function of the runtime
    let start =  std::time::Instant::now();
    rt.block_on(future);
    let end =  std::time::Instant::now();
    log::info!("[CPU INTENSE ASYNCRONOUS - LOW PERF] TOOK: {:?} seconds to run ", end-start);
     
}
