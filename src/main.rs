#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    let builtins = vec!["echo", "exit", "type"];

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let command = buf.trim();
        match command {
            "exit" => break,
            _ if command.starts_with("echo") => {
                let to_echo = command[4..].trim();
                println!("{to_echo}")
            }
            _ if command.starts_with("type") => {
                let to_describe = command[4..].trim();
                if builtins.contains(&to_describe) {
                    println!("{} is a shell builtin", to_describe)
                } else {
                    println!("{}: not found", to_describe)
                }
            }
            _ => println!("{}: command not found", command),
        }
        io::stdout().flush().unwrap();
    }
    exit(0)
}
