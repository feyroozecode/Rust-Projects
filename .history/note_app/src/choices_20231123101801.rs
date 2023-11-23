use std::io;

pub fn get_choice() -> u32 {
    loop {
        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choices)
            .expect("Failed to read line");

        match choice.trim().parse::<i32>() {
            Ok(1..4) => return choices.trim().parse().unwrap(),
            _ => println("Invalid choice enter a number betwen 1 and 3"),
        }
    }
}
/