// Tuples : group together values that can be of different types
//          max 12 values (elements)

pub fn run(){

    let person: (&str, &str, i8)  = ("John" , "Doe", 20);


    println!("My first name is {} and my family name is {} and Im {} years old!", person.0, person.1, person.2);

}