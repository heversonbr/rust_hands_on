// structs
#[allow(dead_code)]

// Ex1: Tradicional Struct 
#[allow(dead_code)]
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}
// -------------------------------------------------------
// Ex2: Tuple Struct
#[allow(dead_code)]
struct TupleColor(u8, u8, u8);


// -------------------------------------------------------
// Ex3: Using structs with functions

    // 1) First we create the struct 
    struct Person { 
        first_name: String,
        last_name: String
    }

    // 2) Then, we will associate functions to it 
    impl Person{
        #[allow(dead_code)]
        fn new(first: &str, last: &str) -> Person {

            Person{
                    first_name: first.to_string(),
                    last_name: last.to_string()
            }
        }
        #[allow(dead_code)]
        fn fullname(&self) -> String {
            // format is a macro, similar to println, except by the fact that it does not print!
            format!("{} {}", self.first_name, self.last_name)
        }

        // if we want to mutate properties
        #[allow(dead_code)]
        fn set_last_name(&mut self, newlast: &str) {

            self.last_name = newlast.to_string();
        }

        // ex return Name to tuple
        #[allow(dead_code)]
        fn to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }
    }
// -------------------------------------------------------
#[allow(dead_code)]
pub fn run(){

    // -------------------------------------------------------
    // Ex1: using tradicional struc
    let mut c = Color{
        red : 255,
        green: 0,
        blue: 0,
    };

    println!("Color: {} {} {} " , c.red, c.green, c.blue );   
    c.red =200;
    println!("Color: {} {} {} " , c.red, c.green, c.blue );  
    // -------------------------------------------------------

    // Ex2: Using tuple struct 
    let mut tuple_color = TupleColor(255,0,0);
    println!("Tuple Color: {} {} {} " , tuple_color.0, tuple_color.1, tuple_color.2 );  
    tuple_color.0 = 150;
    println!("Tuple Color: {} {} {} " , tuple_color.0, tuple_color.1, tuple_color.2 );  
// -------------------------------------------------------

    // Ex3: Using structs with associated functions
    let mut p = Person::new("John", "Doe");
    println!("Person is: {} {} " , p.first_name, p.last_name);
    println!("Fullname: {}", p.fullname() );
    // changing properties (check method set_last_name)
    p.set_last_name("Willians");
    println!("Fullname: {}", p.fullname() );
    println!("Fullname: {:?}", p.to_tuple() );
    // -------------------------------------------------------
}