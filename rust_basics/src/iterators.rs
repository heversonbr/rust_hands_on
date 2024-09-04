// NOTES:
// In Rust, an iterator is a fundamental concept for traversing through collections or sequences of elements.
//  Iterators in Rust are lazy, meaning they don't execute until you explicitly consume them. 
// They provide a safe and efficient way to work with collections without sacrificing performance or safety.

use core::num;
use std::result;

#[allow(dead_code)]
pub fn run(){
    // Exemple 1: 
    //  Here in this example we declare a vector of string slices ('str' type) 
    //  Then, we turn this vector into an iterator in order to access each element
    //  We use the method 'into_iter()' that creates a consuming iterator, that is, 
    //  the iterator that moves each value out of the vector (from start to end). The vector cannot be used after calling this.
    {
        let fruits = vec!["banana", "apple" , "passion-fruit" , "strawberry"];
        let mut fruit_iter = fruits.into_iter();
        println!("Iterator.next(): {:?}", fruit_iter.next());
        println!("Iterator.next(): {:?}", fruit_iter.next());
        println!("Iterator.next(): {:?}", fruit_iter.next());
        println!("Iterator.next(): {:?}", fruit_iter.next());
        println!("Iterator.next(): {:?}", fruit_iter.next());
    }
    println!("------------------------------------------------------");
    // Example 2:
    // Of course, using the method 'next()' to access elements of the iterator is quite verbose
    // An easier way of doing this is to use a for loop as shown above
    {
        let breads = vec!["baguette", "croissant" , "pain_au_chocolat" , "Brioche"];
        for item in breads {
            println!("Item: {:?}", &item); }
    }
    println!("------------------------------------------------------");
    // Example 3: Creating iterators
    // When creating iterators in Rust, choosing between `iter()`, `iter_mut()`, and `into_iter()` 
    // depends on how you want to interact with the elements of the collection.
    // -  `iter()`  when we want to iterate over `immutable` references to the elements in a collection.
    // - `iter_mut()` when we want to iterate over `mutable` references to the elements in a collection.
    // - `into_iter()` when we want to iterate over the owned values of the collection, consuming the collection in the process.
    // NOTE: Consuming a collection means transferring ownership, making the original collection unusable.
    {   
        let numbers = vec![1, 2, 3, 4, 5];
        // Let's create an Iterator of immutable references, we wont take ownership of the vector
        println!("Original vector : {:?}" , numbers );
        let immut_ref_iterator = numbers.iter();
        for item in immut_ref_iterator {
            println!("Iterator of Immutable references - [item : {:?}]" , item );  }
        println!("Original, no owned vector still available: {:?}" , numbers );
    }
    {   
        let mut numbers = vec![1, 2, 3, 4, 5];
        // Let's create an Iterator of mutable references, we wont take ownership of the vector either, just be able to mutate values
        let mut_ref_iterator = numbers.iter_mut();
        for item in mut_ref_iterator {
            println!("Iterator of Immutable references - [item + 1 : {:?}]" , *item + 1 );  }
        println!("Original, no owned vector still available: {:?}" , numbers );
    }
    {   
        let numbers = vec![1, 2, 3, 4, 5];
        // Now lets create an iterator of owned values, from our collection
        let owned_values_iterator = numbers.into_iter();
        // at this point, the original vector was consumed and we cannot access it anymore.
        for item in owned_values_iterator {
            println!("Iterator of owned values - [item + 1 : {:?}]" , item );
        }
        // if we try to use the original vector again, we will have a compiling error.
        // because the ownership of the original vector was moved to the iterator.
        // uncomment the next line to see the error. 
        // println!("Original, no owned vector still available: {:?}" , numbers );
    }
    // Creating an iterator from a range-based sequence of numbers
    {
        // Inclusive range (from 1 to 5, including 5)
        let _range_iter = (1..=5).into_iter();
    }
    // Creating an iterator from an array:
    {
        let mut array = [1, 2, 3, 4, 5];
        // Immutable iteration over array elements
        let _iter1 = array.iter();
        // Mutable iteration over array elements
        let _iter2 = array.iter_mut();
    }


    // NOTE: Iterating Over a String or &str 
    // chars() -> Returns an iterator over the chars of a string slice (&str) or a String
    // 'Borrowing': The chars() method borrows the string (whether it's a &str or a String). 
    // It doesn’t take ownership of the string, IT DOES NOT CONSUMES the original value
    // so the original string is not consumed and remains accessible after the iteration.
    // Example 6: iterating over strings
    {
        let text1 = "Hello";
        let char_iter1 = text1.chars();
        for item in char_iter1 {
            println!("Item in char_iter1: {}", item);
        }
        println!("Original string of char_iter1: {}", text1);
    }
    {
        let text2 = String::from("World!");
        let char_iter2 = text2.chars();
        for item in char_iter2 {
            println!("Item in char_iter2: {}", item);
        }
        println!("Original string of char_iter2: {}", text2);
    }
    
    println!("------------------------------------------------------");
    // ------------------------------------------
    // Useful methods 
    // ------------------------------------------
    //
    // ------------------------------------------
    // Example 4:  method collect() 
    //          collect() : Transforms an iterator into a collection.
    //          collect() can take anything iterable, and turn it into a relevant collection.
    //          The most basic pattern in which collect() is used is to turn one collection into another.
    
    // ------------------------------------------   
    println!("collect() method:");   
    {
        // Creates an iterator that takes ownership of the vector of characters.
        let chars = vec!['R', 'u', 's', 't'];
        // Consumes the iterator and collects the characters into a String.
        let word: String = chars.into_iter().collect();
    }
    // NOTE: 
    // The collect() method is very general because it can convert an iterator into many different 
    // types of collections. This generality can sometimes lead to ambiguity in type inference, where the Rust 
    // compiler cannot determine the exact type of collection you want to create.
    // To resolve such ambiguity, Rust provides a syntax known as the "turbofish" (::<>). 
    // This allows you to explicitly specify the type you want collect() to produce.
    {
        let numbers = vec![1, 2, 3, 4, 5];
        // If we use the next line we will have ambiguity without specifying the type
        // let result = numbers.iter().map(|&x| x * 2).collect(); // Error: type inference needed
        // We need to specify that we want to collect into a Vec<i32> using the turbofish sintax `::<>`
        let result = numbers.iter().map(|&x| x * 2).collect::<Vec<i32>>();
        println!("{:?}", result);
    }

    // ------------------------------------------
    // Example 5: 
    // Chaining interators with 'chain()' method
    // Takes two iterators and creates a new iterator over both in sequence. 
    // ------------------------------------------
    println!("chain() method:");
    {     
        let fruits = vec!["banana", "apple" , "passion-fruit" , "strawberry"];
        let breads = vec!["baguette", "croissant" , "pain_au_chocolat" , "Brioche"];
        println!("fruits iterator: {:?}", fruits);
        println!("breads iterator: {:?}", breads);
        
        // chain() will return a new iterator which will first iterate over values from the first iterator 
        // and then over values from the second iterator. 
        let aggregate_food = fruits.iter().chain(&breads);
        // Since the argument to chain() uses 'IntoIterator', we can pass anything that can be converted into an Iterator, 
        // not just an Iterator itself. For example, slices (&[T]) implement 'IntoIterator', and so can be passed to chain() directly.
        // aggregate_food is the new iterator created by chain with elements of both 'fruits' and 'breads'

        // 'collect()' method transforms an iterator into a collection.
        // collect() can take anything iterable, and turn it into a relevant collection. 
        // This is one of the more powerful methods in the standard library, used in a variety of contexts.
        let all_food: Vec<&&str> = aggregate_food.collect();  
        // we have used clone() here to avoid taking the ownwership of the value,  
        //because we need it for the following loop in the example
        //for food in aggregate_food {
        //    println!("{:?}" , food);    
        //}
        println!("all_food iterator: {:?}", all_food);
    }
    println!("------------------------------------------------------");

    // ------------------------------------------
    // Example 6:  last()
    // Get the last element of an iterator
    // last() method: Consumes the iterator, returning the last element.
    // ------------------------------------------
    println!("last() method:");
    {
        let a = [1, 2, 3, 4, 5];
        let my_iterator =  a.iter();
        let last_element = my_iterator.last().unwrap();
        println!("last_element of iterator: {:?}", last_element);
        // we can access the original iterator, because we create it from references, iter()
        println!("Original iterator: {:?}", a);
        // but we cannot access the iterator after using the method last(), because last consumes the iterator.
        // println!("my_iterator: {:?}", my_iterator);
    }
    println!("------------------------------------------------------");

    // ------------------------------------------
    // Example 7: enumerate()
    // Produces tuples of index and value for each element of the iterator.
    // Creates an iterator which gives the current iteration count as well as the next value.
    // The iterator returned yields pairs (i, val), 
    // where i is the current index of iteration and val is the value returned by the iterator.
    println!("enumerate() method:");
    { 
        let numbers = vec![10, 20, 30];
        for (index, value) in numbers.iter().enumerate() {
            println!("Index: {}, Value: {}", index, value);
        }
    }
    // ------------------------------------------

    println!("------------------------------------------------------");
    
    // ------------------------------------------
    // Example 8: filter()
    // ------------------------------------------
    // - filter(self, Predicate)
    //   Filters elements of the iterator based on a predicate function.
    //   Creates an iterator which uses a closure to determine if an element should be yielded.
    //   Given an element the closure must return true or false. 
    //   The returned iterator will yield only the elements for which the closure returns true.
    println!("filter() method");
    {   
        let numbers = vec![1, 2, 3, 4, 5];
        let even_numbers  = numbers.iter().filter(|&x| x % 2 == 0);
        for item in even_numbers {
            println!("event item on the new iterator(even_numbers) => {:?}" , item);    
        }

    }
    println!("------------------------------------------------------");

    // ------------------------------------------
    // Example 9:  skip() and take() 
    // ------------------------------------------
    // skip() : Creates an iterator that skips the first n elements.
    //          skips elements until n elements are skipped or the end of the iterator is reached (whichever happens first). 
    //          After that, all the remaining elements are yielded.
    //          If the original iterator is too short, then the returned iterator is empty.
    // ------------------------------------------
    println!("skip() method");
    {
        let numbers = vec![1, 2, 3, 4, 5];
        println!("Original collection: {:?}" , numbers);

        let skipped_iter= numbers.iter().skip(3);
        for item in skipped_iter {
            println!("item on the new skipped iterator .skip(3 => {:?}" , item);    
        }

    }
    // ------------------------------------------
    // take() 
    // Take  a specified number of elements from the iterator.
    // Creates an iterator that yields the first n elements, or fewer if the underlying iterator ends sooner.
    // ------------------------------------------
    println!("take() method:");
    {
        let numbers = vec![1, 2, 3, 4, 5];
        println!("Original collection: {:?}" , numbers);
        let first_three = numbers.iter().take(3);
        for item in first_three {
            println!("item taken iterator.take(3)=> {:?}" , item);    
        }

    }
    println!("------------------------------------------------------");
    // ------------------------------------------
    // Example 10: methods all() and any() 
    // ------------------------------------------
    // all() : Checks if all of the elements satisfy a predicate function.
    //         all() takes a closure that returns true or false
    //         It applies this closure to each element of the iterator, and if they all return true, 
    //         then so does all(). If any of them return false, it returns false.
    // NOTE: An empty iterator will always return true.
    println!("all() method:");
    {
        let numbers = vec![1, 2, 3, 4, 5];
        println!("Original collection: {:?}" , numbers);
        let are_all_even = numbers.iter().all(|&x| {x % 2 == 0});
        println!("All items are even ? => {:?} \n" , are_all_even);    

        let text = String::from("Hello, how are you?");
        println!("Original empty collection: {:?}" , text);
        // in order to show that an empty iterator returns true, 
        // I'm filtering the original iterator to yeld only numeric chars, that dont exist in the original iterator
        // this empty iterator is in turn, given to the all() methods to check if the result elements are alphabetic.
        let result = text.chars().filter(|x| x.is_numeric())
                                        .all(|x| x.is_ascii_hexdigit());
        println!("All items are is_ascii_hexdigit? => {:?} \n" , result);    
        // although none of the elements are ascii hexadigit, it replies true        
    }
    // ------------------------------------------
    // any() : 
    //          Checks if any of the elements satisfy a predicate function.
    //          any() takes a closure that returns true or false. It applies this closure to each element of the iterator, 
    //          and if any of them return true, then so does any(). If they all return false, it returns false.
    // NOTE: An empty iterator returns false.
    // ------------------------------------------
    println!("any() method:");
    {
        let numbers = vec![1, 2, 3, 4, 5];
        println!("Original collection: {:?}" , numbers);
        let are_all_even = numbers.iter().any(|&x| {x % 2 == 0});
        println!("Any item is even? => {:?} \n" , are_all_even);   

        let text = String::from("Hello, how are you?");
        println!("Original empty collection: {:?}" , text);
        let result = text.chars().filter(|x| x.is_numeric())
                                        .any(|x| x.is_ascii_alphabetic());
        println!("All items are is_ascii_alphabetic? => {:?} \n" , result); 
        
    }
    println!("------------------------------------------------------");
    // ------------------------------------------
    // Example 11: map()
    // ------------------------------------------
    // map(): It is used to transform each element of an iterator into another value by applying a function to it, 
    //        producing a new iterator with the transformed elements.
    //        map() transforms one iterator into another, by means of its argument: something that implements `FnMut`. 
    //        It produces a 'new iterator' which calls the closure on each element of the original iterator.
    //        If you have an iterator that gives you elements of some type A, and you want an iterator of some other type B, 
    //        you can use map(), passing a closure that takes an A and returns a B.
    println!("map() method");
    {
        let numbers = vec![1, 2, 3, 4, 5];
        println!(": Original collection {:?}", numbers);
        // Create an iterator over the vector, square each number using 'map'
        let squared_numbers = numbers.iter().map(|&x| x * x);
        // Collect the squared numbers into a new vector
        let squared_numbers_vec: Vec<i32> = squared_numbers.collect();
        println!("mapped elements{:?}", squared_numbers_vec); // Output: [1, 4, 9, 16, 25]
    }
    println!("------------------------------------------------------");

    // ------------------------------------------
    // Example 12:  method inspect()
    // inspect() : Does something with each element of an iterator, passing the value on.
    // The inspect() method in Rust is an iterator adapter that allows you to peek at each item as it passes through 
    // the iterator chain without modifying the items. This is particularly useful for debugging or logging the values during iteration.
    // ------------------------------------------
    println!("inspect() method:");
    {
        let numbers = vec![1, 2, 3, 4, 5];
        // Chain of iterator methods with inspect
        let doubled_numbers: Vec<i32> = numbers
            .iter() // Create an iterator over immutable references to the elements
            .inspect(|&x| println!("Original value: {}", x)) // Inspect and print each original value
            .map(|&x| x * 2) // Double each value
            .inspect(|&x| println!("Doubled value: {}", x)) // Inspect and print each doubled value
            .collect(); // Collect the results into a new vector
        
        println!("Resulting vector: {:?}", doubled_numbers); // Output: [2, 4, 6, 8, 10]
    }
   
     // ------------------------------------------
    // Example 13 : zip():  
    // Zips up two iterators into a single iterator of pairs.
    // zip() returns a new iterator that will iterate over two other iterators, 
    //  returning a tuple where the first element comes from the first iterator, 
    //  and the second element comes from the second iterator.
    //  In other words, it zips two iterators together, into a single one.
    // ------------------------------------------
    println!("zip() method:");
    {
        let a1 = [1, 2, 3];
        let a2 = [4, 5, 6];
        
        println!("iterator a1: {:?}" , a1);
        println!("iterator a2: {:?}", a2);
        let iter = a1.iter().zip(a2.iter());
        println!("Zipper iterator ->");
        for item in iter {
            println!("item (from iterator 1, from iterator 2) => {:?}" , item);    
        }
    }
    println!("------------------------------------------------------"); 

}