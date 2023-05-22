use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

#[allow(unused_must_use)]

fn main() {
    loop {
        print!("kai :> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let next_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(next_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let child = Command::new(command).args(args).spawn();
                match child {
                    Ok(mut child) => {
                        child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
