#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{self},
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::exit,
};

fn main() -> io::Result<()> {
    // TODO: Uncomment the code below to pass the first stage
    let builtins = vec!["echo", "exit", "type"];

    // let file = fs::File::create("1.md").unwrap();
    // let meta = fs::metadata("1.md").unwrap();
    // let perms = meta.permissions();
    let mut path_dirs = Vec::new();

    if let Some(path) = std::env::var_os("PATH") {
        for dir in env::split_paths(&path) {
            path_dirs.push(dir);
        }
    }

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
                    let mut found = false;
                    for dir in &path_dirs {
                        if let Some(path) = find_executable(dir, to_describe)? {
                            println!("{} is {}", to_describe, path.display());
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("{}: not found", to_describe)
                    }
                }
            }
            _ => println!("{}: command not found", command),
        }
        io::stdout().flush().unwrap();
    }
    exit(0)
}

fn find_executable<'a>(dir: &Path, name: &'a str) -> io::Result<Option<PathBuf>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        if path.file_name().and_then(|s| s.to_str()) != Some(name) {
            continue;
        }

        let mode = entry.metadata()?.permissions().mode();
        if mode & 0o111 != 0 {
            return Ok(Some(path));
        }
    }

    Ok(None)
}
