#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let vec: Vec<&str> = input.trim().split_whitespace().collect();
        match vec[0] {
            "exit" => {
                break;
            }
            "echo" => {
                println!("{}", vec[1..].join(" "))
            }
            _ => {
                println!("{}: command not found", vec[0]);
            }
        }
    }
}