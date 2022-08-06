use std::process::{Command, Child};
use std::io::{Write, stdout, stdin, Error};
use std::str::SplitWhitespace;
use std::option::Option;

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
                        child.wait();
                        todo!("handle child exit status");
                    },
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}  
