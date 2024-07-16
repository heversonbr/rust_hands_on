// An enumerated type is a nominal, heterogeneous disjoint union type
// enums are types that have few defined values
// An enum item declares both the type and a number of variants, 
//     each of which is independently named and has the syntax of a struct, tuple struct or unit-like struct.
// Ref: https://doc.rust-lang.org/reference/types/enum.html
#[allow(dead_code)]
// Example 1: 
enum Movements{
    // Variants
    Up, 
    Down,
    Right,
    Left
}
#[allow(dead_code)]
fn move_avatar(m: Movements){
    
    // perform action depending on info
    // in this case we use the match pattern to chose the action to take , in function of the enum value.
    match m {
        Movements::Up =>  println!("Avatar moving UP"),
        Movements::Down =>  println!("Avatar moving Down"),
        Movements::Right =>  println!("Avatar moving Right"),
        Movements::Left =>  println!("Avatar moving Left")
    }
}

//Example 2: 
#[derive(Debug)]
#[allow(unused)]
enum MultiTypesElement {
    Inteiro(i32),
    Floating(f64),
    Texto(String),
}

#[allow(dead_code)]
pub fn run(){

    //Example 1: 
    println!("Example 1: ");
    let avatar1 = Movements::Left;
    let avatar2 = Movements::Up;
    let avatar3 = Movements::Right;
    let avatar4 = Movements::Down;

    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    move_avatar(avatar1);

    // Example 2: using enum to store different types in a vector
    println!("Example 2: ");
    let multi_type_vect: Vec<MultiTypesElement> = vec![
        MultiTypesElement::Inteiro(5),
        MultiTypesElement::Floating(3.45),
        MultiTypesElement::Texto(String::from("Teste")),
    ];

    println!("{:?}" , multi_type_vect);

}