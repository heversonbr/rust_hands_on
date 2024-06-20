use tokio::{io::AsyncReadExt, time};

// #2 : The second example runs the program asyncronously. 
// Asyncronous execution :  
async fn async_sleeper(){
    log::info!("[ASYNCRONOUS] Sleeping");
    time::sleep(time::Duration::from_secs(1)).await;
    log::info!("[ASYNCRONOUS] Awake");
}

async fn async_reader(){
    log::info!("[ASYNCRONOUS] Reading some data...");
    let mut f = tokio::fs::File::open("../data.csv").await.expect("Error opening file!");
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


fn main() {


    
    // create the simple logger:
    simple_logger::init_with_level(log::Level::Info).expect("Error creating logger.");
    
    // Run asyncronous example: 
    // create an instance of the tokio run time
    let rt = tokio::runtime::Runtime::new().unwrap();
    // we call our run funtion which returns a Future.
    let future = run_asyncronously(); 
    // we pass the future to the block_on function of the runtime
    let start =  std::time::Instant::now();
    rt.block_on(future);
    let end =  std::time::Instant::now();
    log::info!("[ASYNCRONOUS] TOOK: {:?} seconds to run ", end-start);

}
