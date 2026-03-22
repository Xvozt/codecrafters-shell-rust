#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let command = buf.trim();
        match command {
            "exit" => break,
            _ => println!("{}: command not found", command),
        }
        io::stdout().flush().unwrap();
    }
    exit(0)
}
