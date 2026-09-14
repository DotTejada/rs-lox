use std::{env, fs::read_to_string, io};

mod tokentype;

fn report(line: i32, pos: &String, message: &String) {
    println!("[line {}] Error{}: {}", line, pos, message);
    hadError = true;
}

fn error(line: i32, message: &String) {
    report(line, "", message);
}

fn run(source: &String) {
    let scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scanTokens();

    for token in tokens {
        println!("{}", token);
    }
}

fn runFile(path: &String) {
    let contents = read_to_string(path);
    match contents {
        Ok(source_code) => {
            run(&source_code)
        },
        Err(err) => {
            println!("{:?}", err);
        },
    }
}

fn runPrompt() {
    let mut line = String::new();
    loop {
        print!("> ");
        match io::stdin().read_line(&mut line) {
            Ok(_) => {},
            Err(err) => {
                println!("{:?}", err);
                break;
            },
        }
        run(&line);
        line.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        return;
    } else if args.len() == 2 {
        runFile(&args[1]);
    } else {
        runPrompt();
    }
}
