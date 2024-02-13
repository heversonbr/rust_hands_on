use std::collections::HashMap;


// This is a basic example showing how to use hash maps in Rust 
//  A HashMap<K, V> stores a mapping of keys of type K to values of type V.
//  It does this via a hashing function, which determines how it places these keys and values into memory. (source: rust book, steve klabnik)
//
pub fn run() {


    {

        let mut exams_scores = HashMap::new();  // create a new hash map 
        exams_scores.insert(String::from("Alice"), 85);
        exams_scores.insert(String::from("Bob"), 75);  
        println!("{:?}", exams_scores);   // prints hashmap:  {"Bob": 75, "Alice": 85}

    }

    {
        // Another way of constructing a hash map is by using the collect method on a vector of tuples, where each tuple consists of a key and its value
        // if we had the team names and initial scores in two sep- arate vectors,
        //  we could use the zip method to create a vector of tuples 
        // Then we could use the collect method to turn that vector of tuples into a hash map.
        let students = vec![String::from("Alice"), String::from("Bob")];
        let scores = vec![85, 75];
        println!("Vector students: {:?}", &students);
        println!("Vector scores: {:?}", &scores);
        

        // Creating a hash map from two other vectors 
        // The type annotation HashMap<_, _> is needed here because it’s possible to collect into many different data structures and Rust doesn’t know which you want unless you specify. 
        let exam_scores : HashMap<_, _> = students.iter().zip(scores.iter()).collect();

        println!("Hash Map exam_scores, zipped from previous vectors{:?}: ", exam_scores); 
        
        
        // Accessing Values in a Hash Map
        let score = exam_scores.get(&String::from("Alice"));
        //println!("Alice' score is: {:?}", score);
        println!("Alice' score is: {:?}", score.expect("expected value corresponding to Alice's key"));

        // We can also iterate over each key/value pair in a hash map in a similar manner as we do with vectors:
        for (key, value) in exam_scores{
            println!("key: {:?} , value: {:?}" , key, value);

        }

    }
    
}