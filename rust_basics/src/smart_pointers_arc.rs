use std::sync::Arc;

// Rc<T> is only for use in single-threaded scenarios, now let check how to use smart pointers in multi-threaded scenarios.
// 
// Lets use the preivious example if Trucks and Facilities.
// We supose that we have a struct that represents a truck with his corresponding capacity. 
// Difference trucks will  have different capacities. 
// We also have different facilities and each facility receives a specific set of trucks.
// 
// For some reason, we want to hand these facilities off to different threads. 
// The reason for doing this, does not matter in this example, it is just for pedagogical reasons. 
// 
// The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. 
// Invoking clone on Arc produces a new Arc instance, which points to the same allocation 
// on the heap as the source Arc, while increasing a reference count. When the last Arc pointer
//  to a given allocation is destroyed, the value stored in that allocation (often referred to as
//  “inner value”) is also dropped.

// We have the Truck struct 
#[derive(Debug)]
#[allow(unused)]
struct Truck {
    capacity: i32,
}

#[allow(dead_code)]
pub fn run(){

// We instantiate 3 different trucks with their specific capacities.
    let truck_a = Arc::new(Truck{ capacity: 1});
    let truck_b = Arc::new(Truck{ capacity: 2});
    let truck_c = Arc::new(Truck{ capacity: 3}); 

// Now lets suppose that we have different facilities and each facility receives a set of trucks. 
// Then we will hand them off to different threads. We will use the spawn method to create these facilities in a thread. 
// If we try to create the thread by using the Rc smart pointer, we will get an error saying 

// "Rc<Truck>` cannot be sent between threads safely. The trait `Send` is not implemented for `Rc<Truck>`, 
// In fact, the Send Trait is implement any data structure that is safe to the sent to another thread. 
// Rc does not implement Send, and for that reason we only use RC in single thread situations.
// In order to safely send a data structure to another thread we have to use the Arc smart pointer. 
// Arc: atomic reference counting . Arc does implement Send and therefore is safe. 
// Arc does not use locks or mutex, it actually uses ADT (atomic data type)

    let thread = std::thread::spawn(move || {

        let facility_one: Vec<Arc<Truck>> = vec![ Arc::clone(&truck_a), Arc::clone(&truck_b)];
        let facility_two: Vec<Arc<Truck>> = vec![ Arc::clone(&truck_b), Arc::clone(&truck_c)];
        (facility_one, facility_two)
    });

    let (facility_one, facility_two) = thread.join().unwrap();

    println!("[Arc smart pointer] Facility one: {:?}" , facility_one);
    println!("[Arc smart pointer] Facility two: {:?}" , facility_two);

    // count the number of references with strong_count method
    // note that we need to clone the truck_b again from either facility one or facility two because the ownership of truck_b 
    // was transfered to the thread.  So in order to retrieve truck_b again we clone that from facility one or two.
    let truck_b = Arc::clone(&facility_one[1]);
    println!("[Arc smart pointer] # of references `truck b` (strong count): {:?}" , Arc::strong_count(&truck_b));
    
    // drop one of the references of 'truck_b', by dropping (closing the scope) of facility_two
    std::mem::drop(facility_two);

    println!("[Arc smart pointer] Facility one after : {:?}" , facility_one);
    // if we try to print facility_two, as shown in the next line, we will get a compile error because facility_two is now out of scope.
    // println!("Facility two: {:?}" , facility_two);

    // count the number of references with strong_count method, after dropping
    println!("[Arc smart pointer] # of references `truck b` (strong count): {:?}" , Arc::strong_count(&truck_b));

}