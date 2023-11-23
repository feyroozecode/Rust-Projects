use std::io;

enum Operation {
    ADD,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

/// A package for my calc app
pub fn calc_app() {
    println!("Ahlan wa Sahlan to Zakiy Artihemetics Calculator !");

    let num1 = get_number("Enter le  premier nombre : ");
    let num2 = get_number("Enter le second mombre : ");

    println!("Choisissez L'operation : ");
    println!("1: Addition");
    println!("2: Soustraction");
    println!("3: Multiplication");
    println!("4: Division ");

    let choice = get_choice();

    // perform operation
    match choice {
        1 => perform_calc(Operation::ADD, num1, num2),
        2 => perform_calc(Operation::MINUS, num1, num2),
        3 => perform_calc(Operation::MULTIPLY, num1, num2),
        4 => perform_calc(Operation::DIVIDE, num1, num2),
        _ => println!("Choix invalide !"),
    }
}

// get a number
pub fn get_number(prompt: &str) -> f64 {

    loop {
        println!("{}", prompt);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");


            match input.trim().parse::<f64>(){
                Ok(num) => return num,
                Err(_)  => println!("Invalid input, enter a valid number"),
            }
        }
    }
     
fn get_choice() -> u32 {    
    loop {
        
    }   
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse::<i32>() {
            Ok(1..=4) => return choice.trim().parse().unwrap(),
            _ => println!("Invalid choice enter a number betwen 1 and 4."),
        }
}

/**
 * @param => op: Operation to perform
 * @param => num1: First number to tack
 * @param => num2: Second number to tack
 */
fn perform_calc(op: Operation, num1: f64, num2: f64) {
    let result = match op {
        Operation::ADD => num1 + num2,
        Operation::MINUS => num1 - num2,
        Operation::MULTIPLY => num1 * num2,
        Operation::DIVIDE => {
            if num2 != 0.0 {
                num1 / num2
            } else {
                println!("Division by zero not allowed");
                return;
            }
        }
    };

    println!("Resultat {}", result)
}

// get arithemetic operator
