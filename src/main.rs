use std::{env, fs::read_to_string, io::{self, Write}};

use rs_lox::{interpreter::Interpreter, parser::Parser, scanner::Scanner, token::Token};

struct Lox {
    interpreter: Interpreter
}

impl Lox {

    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new()
        }
    }

    fn run(&mut self, source: Vec<char>) -> Result<(), Box<dyn std::error::Error>> {

        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;

        let mut parser: Parser = Parser::new(tokens);
        let statements = parser.parse()?;

        self.interpreter.interpret(statements)?;

        Ok(())
    }

    fn run_file(&mut self, path: &String) {
        let contents = read_to_string(path);
        match contents {
            Ok(source_code) => {
                let source: Vec<char> = source_code.chars().collect();
                match self.run(source) {
                    Ok(()) => {},
                    Err(err) => {
                        println!("{}", err);
                    }
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
            let char_vec: Vec<char> = line.trim().chars().collect();
            match self.run(char_vec) {
                Ok(()) => {},
                Err(err) => {
                    println!("{}", err);
                    break;
                }
            }
            line.clear();
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: cargo run [script]");
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
