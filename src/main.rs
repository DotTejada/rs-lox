use std::{env, fs::read_to_string, io::{self, Write}};

use rs_lox::{expr::AstPrinter, interpreter::Interpreter, parser::Parser, scanner::Scanner, token::Token};

fn run(source: Vec<char>) -> Result<(), Box<dyn std::error::Error>> {

    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens()?;
    for token in &tokens {
        println!("{:?}", token);
    }

    let mut parser: Parser = Parser::new(tokens);
    let expression = parser.parse()?;
    println!("{}", AstPrinter{}.print(&*expression));

    let interpreter: Interpreter = Interpreter{};
    println!("{}", interpreter.interpret(expression)?);

    Ok(())
}

fn run_file(path: &String) {
    let contents = read_to_string(path);
    match contents {
        Ok(source_code) => {
            let source: Vec<char> = source_code.chars().collect();
            match run(source) {
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

fn run_prompt() {
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
        match run(char_vec) {
            Ok(()) => {},
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
        line.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: cargo run [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
