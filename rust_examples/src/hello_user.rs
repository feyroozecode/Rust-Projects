/// imports io 
use std::io; 

/// Declare a rust package
pub fn start_app() {

    // Create a mutable variable to store user input
    let mut input: String = String::new();

    // Prompt user to enter a a string
    println!("Entrer votre nom : ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let user_name: String = input
        .trim().parse()
        .expect("Invalid input for string");

    input.clear();  // Clear the input

    // enter age 
    println!("Entrer votre age : ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let user_age: i32 = input.trim().parse().expect("Invalid input for integer");
    input.clear();

    // enter a weight 
    println!("Entrer votre poid (k/g) : ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let user_weight: i64 = input.trim().parse().expect("Invalid input for float ");  

    let _message = format!("Assalam Aleykoum , Hi {} , tu  est agé de {} ans, et pése : {} kg", user_name, user_age, user_weight );
    println!("{}", _message);
    
}