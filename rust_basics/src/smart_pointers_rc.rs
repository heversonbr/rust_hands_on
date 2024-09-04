use std::rc::Rc;

// Single-threaded reference-counting pointers. ‘Rc’ stands for ‘Reference Counted’.
// The type Rc<T> provides shared ownership of a value of type T, allocated in the heap.

// In the majority of cases, ownership is clear: you know exactly which variable owns a given value. 
// However, there are cases when a single value might have multiple owners.
// We have to enable multiple ownership explicitly by using the Rust type Rc<T>, which is an abbreviation for reference counting.
// The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. 
// If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read 
// and we can’t determine at compile time which part will finish using the data last. 
// 
// Example: 
// Supose that we have a struct that represents a truck with his corresponding capacity. 
// Difference trucks have different capacities. 

#[derive(Debug)]
#[allow(unused)]
struct Truck {
    capacity: i32,
}

#[allow(dead_code)]
pub fn run(){

// We instantiate 3 different trucks with their specific capacities.
// each of these references will point to the value corresponding to each truck.
// We instantiate each truck as a Rc<T> in order to drop the value from the memory 
// as soon as the last reference to it goes out of scope.
    let truck_a = Rc::new(Truck{ capacity: 1});
    let truck_b = Rc::new(Truck{ capacity: 2});
    let truck_c = Rc::new(Truck{ capacity: 3}); 

// Now lets suppose that we have different facilities and each facility receives a set of trucks. 
// We will use vectors to map a set of trucks to each facility. 
    let facility_one: Vec<Rc<Truck>> = vec![ Rc::clone(&truck_a), Rc::clone(&truck_b)];
    let facility_two: Vec<Rc<Truck>> = vec![ Rc::clone(&truck_b),  Rc::clone(&truck_c)];

    println!("Facility one: {:?}" , facility_one);
    println!("Facility two: {:?}" , facility_two);

    // count the number of references with strong_count method
    println!("# of references `truck b` (strong count): {:?}" , Rc::strong_count(&truck_b));
    
    // drop one of the references of 'truck_b', by dropping (closing the scope) of facility_two
    std::mem::drop(facility_two);

    println!("Facility one after : {:?}" , facility_one);
    // if we try to print facility_two, as shown in the next line, 
    // we will get a compile error because facility_two is now out of scope.
    // println!("Facility two: {:?}" , facility_two);

    // count the number of references with strong_count method, after dropping
    println!("# of references `truck b` (strong count): {:?}" , Rc::strong_count(&truck_b));


}