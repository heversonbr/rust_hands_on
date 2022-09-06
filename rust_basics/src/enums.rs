// enums are types that have few definite values
// An enum item declares both the type and a number of variants, 
//     each of which is independently named and has the syntax of a struct, tuple struct or unit-like struct.
// Ref: https://doc.rust-lang.org/reference/types/enum.html
#[allow(dead_code)]

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

    match m {
        Movements::Up =>  println!("Avatar moving UP"),
        Movements::Down =>  println!("Avatar moving Down"),
        Movements::Right =>  println!("Avatar moving Right"),
        Movements::Left =>  println!("Avatar moving Left")
    }
}
#[allow(dead_code)]
pub fn run(){


    let avatar1 = Movements::Left;
    let avatar2 = Movements::Up;
    let avatar3 = Movements::Right;
    let avatar4 = Movements::Down;

    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    move_avatar(avatar1);

}