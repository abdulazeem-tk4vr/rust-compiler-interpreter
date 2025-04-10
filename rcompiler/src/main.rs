use std::env;
use std::fs;
use std::io::{self, Write};

fn run_file(file_path: &String) {
    println!(
        "The incoming file that needs to be parsed is {:?}",
        file_path
    );
    let bytes: Vec<u8> = fs::read(file_path).expect("failed to read incoming file");
    let string = String::from_utf8(bytes).expect("Our bytes should be valid utf8");

    run(&string);
}

fn run(source: &String) {
    let tokens: Vec<&str> = source.split_whitespace().collect();

    for token in tokens {
        println!("the token is {}", token);
    }
}


fn main() {
    println!("Hello, world!");
    println!("We are starting the interpreter for the Language Lox");

    // logic starts here
    // collecting command line arguments when running the file
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1]; // &args[0] gives the name of the binary
        run_file(file_path);
    } else {
        println!("We are now going to use command line only");
        let mut input = String::new();

        loop {
            print!(">... ");
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            // Trim to remove newline, check for empty input or quit command
            let trimmed = input.trim().to_string();

            // unexpected new line handling
            if trimmed.is_empty() {
                let mut new_input = String::new();
                println!("Are you sure you want to quit? Y/N");
                io::stdin()
                    .read_line(&mut new_input)
                    .expect("Failed to read line");

                let new_trimmed = new_input.trim();

                if new_trimmed == "Y" {
                    break;
                } else {
                    continue;
                }
            }

            // exiting or quitting the command line flow
            if trimmed == "exit" || trimmed == "quit" {
                break;
            }

            run(&trimmed); 
        }
    }
}
