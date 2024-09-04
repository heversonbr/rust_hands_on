use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
struct Person{
    name: String
}

#[allow(dead_code)]
#[derive(Debug)]
struct AnotherPerson {
    name: RefCell<String>
}

#[allow(dead_code)]
pub fn run(){

    // If we want to change the value owned by the variable p1, which is an instance of Person
    // we need to declare p1 as mutable 
    {
        let mut p1 = Person { name: String::from("Alice") };
        println!("name is {:?}",  p1.name );
        p1.name = String::from("Bob");
        println!("name is {:?}",  p1.name );
    }

    // If we use a RefCell, we can use its Interior Mutability to change the value
    // and we dont need to declare p1 as mutable in order to change it.
    {
        let p1 = AnotherPerson{
            name: RefCell::new(String::from("Charlie"))
        };
        println!("name [using RefCell] is {:?}",  p1.name.take());
        p1.name.replace(String::from("David"));
        println!("name [using RefCell] is {:?}",  p1.name.take() );
    }

    
}