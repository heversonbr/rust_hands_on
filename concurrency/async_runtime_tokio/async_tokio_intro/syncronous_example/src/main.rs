use std::io::Read;

// #1 : The first example runs the program syncronously. 
// Syncronous execution :  
fn sync_sleeper(){
    log::info!("[SYNCRONOUS] Sleeping");
    std::thread::sleep(std::time::Duration::from_secs(1));
    log::info!("[SYNCRONOUS] Awake");
}

fn sync_reader(){
    log::info!("[SYNCRONOUS] Reading some data...");
    let mut f1= std::fs::File::open("../data.csv").unwrap();
    let mut contents = String::new();
    f1.read_to_string(&mut contents).unwrap();
    log::info!("[SYNCRONOUS] Just read {} bytes from file" , contents.len());
}


fn main() {
    
    // create the simple logger:
    simple_logger::init_with_level(log::Level::Info).expect("Error creating logger.");
    
    // Run syncronous example: run syncronously,  the sleeper will only sleep after all reading happens.
    let start =  std::time::Instant::now();
    for _ in 0..10 { 
        sync_reader();
    }
    sync_sleeper();
    let end =  std::time::Instant::now();
    log::info!("[SYNCRONOUS] TOOK: {:?} seconds to run ", end-start);
        
    
}
