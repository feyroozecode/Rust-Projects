use std::io;

pub fn flow_control() {
    voting_system()
}

fn voting_system() {
    let min_age = 18;
let max_age = 85;

    let mut str_age: String = String::new();

    // get an age
    println!("You entered : ");

    io::stdin().read_line(&mut str_age).expect("Failed to read line");

    // convert age to integer
    let age : i16 = str_age.trim().parse().expect("Please enter a number");


    // validate user to vote
    if age > 0 && age < min_age {
        println!(
            "You are not eligible to vote, minmum age to vote is {}",
            min_age
        );
    } else if age > min_age && age < min_age {
        println!("You are eligible to vote");
    } else if age > max_age {
        println!("You are not eligible to vote, MAX age is {} ", max_age);
    } else {
        println!("Unexpected Value enter");
    }
}
