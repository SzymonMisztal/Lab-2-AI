use std::io;
use std::io::Write;

pub fn get_input(prompt: &str) -> String{
    print!("{}",prompt);
    io::stdout().flush();
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}