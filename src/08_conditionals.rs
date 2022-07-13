// Conditionals are used to check the 'condition' of somenthing and act on the result.. 


pub fn run(){


    let age = 18 ;
    let check_id: bool = false;

    // IF ELSE:
    if age >= 21 {
        println!("Batender1 asks : what you'd like to drink?");
    } else{
        println!("Batender1 says: sorry! you have to leave!!!")
    }

    // IF ELSE with more variables
    if age >= 21 && check_id {
        println!("Batender2 asks: what you'd like to drink?");
    } else if age < 21 && check_id{
        println!("Batender2 says: sorry! you have to leave!!!")
    } else{
        println!("Batender2 says: sorry! you need to show your ID!!!")
    }

    let is_known_major = false;
    if age >= 21 && check_id || is_known_major {
        println!("Batender3 asks: what you'd like to drink?");
    } else if age < 21 && check_id{
        println!("Batender3 says: sorry! you have to leave!!!")
    } else{
        println!("Batender3 says: sorry! you need to show your ID!!!")
    }


    // There is no ternary operator in RUST, such as in other programming languages. 
    // There is a shorthand if, though:
    let is_of_age = if age >= 21 {true} else {false};   
    println!("Is of Age: {} ", is_of_age);





}