use std::io;

pub fn get_choice() -> u32 {
    loop {
        let mut choice: String = String::new();

        io::stdin()
            .read_line(&mut choices)
            .expect("Failed to read line");

        match choice.trin().parse
    }
}
