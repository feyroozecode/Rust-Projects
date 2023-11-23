use std::io;

pub fn flow_control() {
    voting_system()
}


fn voting_system() {
let MIN_AGE = 18;
    let MAX_AGE = 85;

    let mut age: i16;

    // get an age
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    // validate user to vote
    if age > 0 && age < MIN_AGE  {
        println!("You are not eligible to vote");
    }
    else if age > MIN_AGE && age < MAX_AGE  {
        println!("You are eligible to vote");
    }

}

