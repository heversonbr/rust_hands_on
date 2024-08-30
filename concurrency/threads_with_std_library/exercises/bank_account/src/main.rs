use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}};

// We are using 'rand' crate to generate random values to make deposits and withdraws
//use rand::Rng;


fn main() {

    // Shared bank account
    // Arc: shared ownership 
    // Mutex: Mutual exclusion for safe mutable access. 
    //        The Mutex ensures that the balance is only modified by one thread at a time, 
    //        preventing race conditions.
    let account_balance:Arc<Mutex<f64>> = Arc::new(Mutex::new(100.0));

    // JoinHandles
    let mut handles = vec![];

    // Spawn 3 threads that deposit money and 3 threads that withdraw money.
    // deposit threads 
    for d in 1..=3 {
        let account_balance_shared_ref = Arc::clone(&account_balance);
        //let value:f64 = rand::thread_rng().gen_range(0.0..100.0);
        let value:f64 = 10.0;

        let handle:JoinHandle<()> = thread::spawn(move || {
            let mut balance = account_balance_shared_ref.lock().unwrap();
            println!("Thread-D{} , CURRENT: {:.2} , DEPOSIT: {:.2} ", d, balance, value);
            *balance += value; 
            // The lock is automatically released when the MutexGuard goes out of scope
        });
        handles.push(handle);
    }

    // withdraw threads
    for w in 1..=3 {
        let account_balance_shared_ref = Arc::clone(&account_balance);
        //let value:f64 = rand::thread_rng().gen_range(0.0..100.0);
        let value:f64 = f64::from(w) * 10.0;

        let handle = thread::spawn(move || {
                let mut balance = account_balance_shared_ref.lock().unwrap();
                println!("Thread-W{} , CURRENT: {:.2} , WITHDRAW: {:.2} ", w, balance, value);
                if *balance >= value {
                    *balance -= value;
                }else{
                    eprintln!("Thread-W{}, Not enough money to withdraw {:.2} ", w, value);
                }
            // The lock is automatically released when the MutexGuard goes out of scope
        });
        handles.push(handle);
    }


    // join the handles
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final Balance : {:.2} ", account_balance.lock().unwrap());

}
