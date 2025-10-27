use std::fs;
use std::io;

fn main() {
    let mut input_pass = String::new();
    io::stdin().read_line(&mut input_pass).unwrap();

    let trimmed_pass = input_pass.trim();

    match fs::read(trimmed_pass) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
