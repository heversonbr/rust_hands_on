// the match expression is a powerful construct that allows you to match 
// a value against a series of patterns and execute code based on the 
// pattern that matches.  
// Match is usuful to match Option enums:   
#[allow(dead_code)]
pub fn run(){ 

    // Example 1:
    println!("Using match: ");
    let x: Option<i32> = Some(5);
    match x {
        Some(value) => {
            println!("The value is {}", value);
        },
        None => {
            println!("The option is empty");
        }
    }
    println!("The x is {:?}", x);

// the x is the  'expression', the 'value', that we want to match on
// Some(value) and None are the patterns we want to match our 'x' against on. 




// the if let syntax can be used as a shorthand for a match expression 
// that only handles one case.  It allows you to match on an expression 
// and then bind a variable to a sub-pattern within that expression, 
// all in one line. The syntax of if let is as follows:
//
//   if let Some(pattern) = expression {
//       code to execute if expression matches Some(pattern)
//   } else {
//       code to execute if expression does not match Some(pattern)
//   }
    println!("Using if let: ");
    let x: Option<i32> = Some(15);
    if let Some(value) = x {
        println!("The value is {}", value);
    } else {
        println!("The option is empty");
    }

    // example using while let :
    let numbers: Vec<Option<i32>> = vec![Some(1),Some(2),Some(3),Some(4),Some(5)];
    println!("The Vector is {:?}", numbers);
    let mut index = 0;
    while let Some(x) = numbers[index]  {
        println!("Vector element is {:?}", x);
        index += 1;
        if index >= numbers.len() {  //this is just to avoid 'index out of bounds'
            break;
        }
    }

}