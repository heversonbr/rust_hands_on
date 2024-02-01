
#[allow(dead_code)]
pub fn test_iterators(){

    {
        let fruits = vec!["banana", "apple" , "passion-fruit" , "strawberry"];
        let mut fruit_iter = fruits.into_iter();
        println!("{:?}", fruit_iter.next());
        println!("{:?}", fruit_iter.next());
        println!("{:?}", fruit_iter.next());
        println!("{:?}", fruit_iter.next());
        println!("{:?}", fruit_iter.next());

    }

    {
        let breads = vec!["baguette", "croissant" , "pain_au_chocolat" , "Brioche"];
        for item in breads {
            println!("{:?}", &item);
        }
    }

    {  // chaining interators

        let fruits = vec!["banana", "apple" , "passion-fruit" , "strawberry"];
        let breads = vec!["baguette", "croissant" , "pain_au_chocolat" , "Brioche"];
        let aggregate_food = fruits.iter().chain(&breads);
        // we can also take an iterator and turn it back into a collection
        let all_food: Vec<&&str> = aggregate_food.clone().collect();  
        // we just use clone here to avoid taking the ownwership of the value, because we need it for the following loop in the example
        for food in aggregate_food {
            println!("{:?}" , food);    
        }
        println!("{:?}", all_food);
    }


  
}