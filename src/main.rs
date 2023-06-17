use std::{
    io::{stdin, stdout, Write},
    process::Command,
};

fn main() {
    loop {
        let mut stdout = stdout();

        print!("% ");
        stdout.flush().unwrap();

        let mut input = String::new();
        let result = stdin().read_line(&mut input);
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        if result.is_err() {
            println!("[error] unable to grab input");
        }

        let child = Command::new(command).args(args).spawn();

        match child {
            Ok(mut child) => {
                child.wait().unwrap();
            }
            Err(err) => {
                println!("{}: {}", command, err);
            }
        }
    }
}
