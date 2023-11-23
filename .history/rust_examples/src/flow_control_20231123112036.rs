use std::io;

pub fn flow_control() {

    voting_system()
}


fn voting_system(){
    let mut age: i16;

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    if age < 18 {
        println!("You are not eligible to vote");
    }
    else if age == 18 {
        println!("You are eligible to vote");
    })
    }

}

