use std::{env, fs::read_to_string, io::{self, Write}};

use crate::{scanner::Scanner, token::Token};

mod tokentype;
mod token;
mod scanner;
mod error;

struct Lox {
    had_error: bool
}

impl Lox {

    fn new() -> Self {
        Self {
            had_error: false
        }
    }

    pub fn report(&mut self, line: i32, pos: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, pos, message);
        self.had_error = true;
    }
    
    pub fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn run(&mut self, source: Vec<char>) {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: &Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    fn run_file(&mut self, path: &String) {
        let contents = read_to_string(path);
        match contents {
            Ok(source_code) => {
                let source: Vec<char> = source_code.chars().collect();
                self.run(source);
                if self.had_error {
                    return;
                }
            },
            Err(err) => {
                println!("{:?}", err);
            },
        }
    }

    fn run_prompt(&mut self) {
        let mut line = String::new();
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            match io::stdin().read_line(&mut line) {
                Ok(n) => {
                    if n == 0 {
                        println!();
                        break;
                    }
                },
                Err(err) => {
                    println!("{:?}", err);
                    break;
                },
            }
            let char_vec: Vec<char> = line.chars().collect();
            self.run(char_vec);
            self.had_error = false;
            line.clear();
        }
    }
}

fn main() {
    let mut lox = Lox::new();
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        return;
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
