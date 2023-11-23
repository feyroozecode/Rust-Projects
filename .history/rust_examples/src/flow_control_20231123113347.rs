use std::io;

pub fn flow_control() {
    voting_system()
}

fn voting_system() {
    let min_age = 18;
    let MAX_AGE = 85;

    let mut str_age: String = String::new();

    // get an age
    io::stdin().read_line(&mut str_age).expect("Failed to read line");

    // convert age to integer
    let age : i16 = str_age.trim().parse().expect("Please enter a number");

    // validate user to vote
    if age > 0 && age < MIN_AGE {
        println!(
            "You are not eligible to vote, minmum age to vote is {}",
            MIN_AGE
        );
    } else if age > MIN_AGE && age < MAX_AGE {
        println!("You are eligible to vote");
    } else if age > MAX_AGE {
        println!("You are not eligible to vote, MAX age is {} ", MAX_AGE);
    } else {
        println!("Unexpected Value enter");
    }
}
