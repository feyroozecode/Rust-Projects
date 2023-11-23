use std::io;

pub fn flow_control() {
    let MIN_AGE = 18;
    let MAX_AGE = 85;

    voting_system()
}


fn voting_system(age: i32) {
    let mut age: i16;

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    if age < 18 {
        println!("You are not eligible to vote");
    }
    else if age > 18 && age < 85  {
        println!("You are eligible to vote");
    }

}

