#![allow(unused_variables)]
use std::{process::{Command, Child}, io::{Write, stdout, stdin, Error}, str::SplitWhitespace};

fn windows_main() {
    
}

fn linux_main() {
    
}

fn main() {
    loop {
        print!("enter command: ");
        stdout().flush().expect("failed to flush stdout");

        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        
        let mut parts: SplitWhitespace = input.trim().split_whitespace();
        let command: &str = parts.next().expect("No command");
        let args: SplitWhitespace = parts;
        
        match command {
            "cd" => {
                todo!("change directory");
            },
            "exit" => { return; },
            _ => {
                let child: Result<Child, Error> = Command::new(command)
                    .args(args)
                    .spawn();
                match child {
                    Ok(mut child) => {
                        drop(child.wait());
                        // todo!("handle child exit status");
                    },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}  
