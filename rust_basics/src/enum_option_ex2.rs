// This is another example exploring 'enums', the 'option' enum specifically
// it explores the methods to manipulate Option, provided by the standard library 

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run(){

    println!("-------------------------------------");
    {
        let value: Option<u32> = Some(10);
        // Returns true if the option is a Some value.
        println!("Exemple 1: Method 'is_some()");
        println!("Method 'is_some()' => must print [true]: {}", value.is_some());
        println!("-------------------------------------");
        // ----------------------------------------------
        // Returns true if the option is a None value.
        println!("Exemple 2: Method 'is_none()'");
        println!("Method 'is_none()' => must print [false]: {}", value.is_none());
    }
    println!("-------------------------------------");
    // ----------------------------------------------
    // expect method: 
    //  Returns the contained Some value, consuming the self value.
    //  Panics if the value is a None with a custom panic message provided by msg.
    {
        println!("Exemple 3: Method 'expect()");
        let x = Some("option_Value");
        let y = x.expect("this is the message in case of x is None");
        println!("Method 'expect()' => This is the valeu removed from the Option with expected method: {}" , y );
        assert_eq!(x.expect("fruits are healthy"), "option_Value");
    }
    // In the code below the 'expect' method panics with `panic message` 
    // because the value of x is None. 
    // let x: Option<&str> = None;
    // x.expect("panic message"); 
    println!("-------------------------------------");
    // ----------------------------------------------
    // unwrap method: 
    //   Returns the contained Some value, consuming the self value.
    //   Because this function may panic, its use is generally discouraged.
    //   Instead, prefer to use pattern matching and handle the None case explicitly
    //   or call unwrap_or, unwrap_or_else, or unwrap_or_default.
    //   Panics if the self value equals None.
    {
        println!("Exemple 4: Method 'unwrap()'");
        let value = Some(1);
        let result = value.unwrap();
        println!("Method 'unwrap()()' => must print value contained in Some : {}", result);
    }
    // In the code below the 'unwrap' method panics with `panic message` 
    // because the value of x is None. 
    //let x: Option<&str> = None;
    //let y = x.unwrap();
    println!("-------------------------------------");
    // ----------------------------------------------
    // take method:   pub fn take(&mut self) -> Option<T>
    //  Takes the value out of the option, leaving a None in its place.
    {   
        println!("Exemple 5: 'take()' method" );
        let mut x = Some(2);
        println!("let mut x = Some(2)' => x = {:?}", x);
        //println!("let y = x.take();");
        let y = x.take();
        println!("let y = x.take();    => y = {:?}", y);
        println!("    x after take()   => x = {:?}", x);
        //assert_eq!(x, None);
        //assert_eq!(y, Some(2));
    }
    println!("-------------------------------------");
    // ----------------------------------------------
    // replace method:  pub fn replace(&mut self, value: T) -> Option<T>
    //   Replaces the actual value in the option by the value given in parameter, 
    //   returning the old value if present, 
    //   leaving a Some in its place without deinitializing either one.
    {
        let mut x = Some(10);
        println!("let mut x = Some(10)'   => x = {:?}", x);
        let y = x.replace(20);
        println!("let y = x.replace(20);");
        println!("    x after replace()   => x = {:?}", x);
        println!("    y after replace()   => y = {:?}", y);
    }
    println!("-------------------------------------");

    // ----------------------------------------------


    let x = 5;
    println!("The value of x before all is: {x}");
    {
        let x = x + 1;
        println!("The value of x is: {x}");
        {
            println!("The value of x in the inner scope before is: {x}");
            let x = x * 2;
            println!("The value of x in the inner scope after is: {x}");
        }
        println!("The value of x is: {x}");
    } 
    println!("The value of x after all is: {x}");

}