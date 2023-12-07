use std::io::{self, Write};

pub fn handle_user_input() -> bool {
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();
    if trimmed == "q" {
        println!("Quitting the program.");
        false
    } else {
        println!("You pressed '{}'.", trimmed);
        // Actions based on user input go here
        true
    }
}
