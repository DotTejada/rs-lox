use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, interpreter::{Callable, Interpreter, Value}, loxfunction::LoxFunction, loxinstance::LoxInstance};

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
    superclass: Option<Rc<LoxClass>>,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: String, superclass: Option<Rc<LoxClass>>, methods: HashMap<String, LoxFunction>) -> Rc<Self> {
        Rc::new(
            Self {
                name,
                superclass,
                methods,
            }
        )
    }

    pub fn find_method(&self, name: &String) -> Option<LoxFunction> {
        if let Some(found) = self.methods.get(name) {
            return Some(found).cloned();
        }
        if let Some(sc) = &self.superclass {
            return sc.find_method(name);
        }
        None
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Callable for Rc<LoxClass> {
    fn arity(&self) -> usize {
        match self.find_method(&"init".to_string()) {
            Some(init) => {
                init.arity()
            },
            None => 0
        }
    }
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value, RuntimeError> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(Rc::clone(self))));
        let initializer = self.find_method(&"init".to_string());
        if let Some(init) = initializer {
            init.bind(instance.clone()).call(interpreter, arguments)?;
        };
        Ok(Value::LoxInstance(instance))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{parser::Parser, resolver::Resolver, scanner::Scanner, token::Token};

    #[test]
    fn devonshire_cream() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: devonshire_cream =====");
        dbg!();
        let source_string = String::from(
"class DevonshireCream {
  serveOn() {
    return \"Scones\";
  }
}

print DevonshireCream; // Prints \"DevonshireCream\".
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("DevonshireCream\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn bagel() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: bagel =====");
        dbg!();
        let source_string = String::from(
"class Bagel {}
var bagel = Bagel();
print bagel; // Prints \"Bagel instance\".
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Bagel instance\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn bacon() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: bacon =====");
        dbg!();
        let source_string = String::from(
"class Bacon {
  eat() {
    print \"Crunch crunch crunch!\";
  }
}

Bacon().eat(); // Prints \"Crunch crunch crunch!\".
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Crunch crunch crunch!\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn egotist() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: egotist =====");
        dbg!();
        let source_string = String::from(
"class Egotist {
  speak() {
    print this;
  }
}

var method = Egotist().speak;
method();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Egotist instance\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn cake() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: cake =====");
        dbg!();
        let source_string = String::from(
"class Cake {
  taste() {
    var adjective = \"delicious\";
    print \"The \" + this.flavor + \" cake is \" + adjective + \"!\";
  }
}

var cake = Cake();
cake.flavor = \"German chocolate\";
cake.taste(); // Prints \"The German chocolate cake is delicious!\".
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("The German chocolate cake is delicious!\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn thing() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: thing =====");
        dbg!();
        let source_string = String::from(
"class Thing {
  getCallback() {
    fun localFunction() {
      print this;
    }

    return localFunction;
  }
}

var callback = Thing().getCallback();
callback();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Thing instance\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn person() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: person =====");
        dbg!();
        let source_string = String::from(
"class Person {
  sayName() {
    print this.name;
  }
}

var jane = Person();
jane.name = \"Jane\";

var bill = Person();
bill.name = \"Bill\";

bill.sayName = jane.sayName;
bill.sayName(); // ?
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Jane\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn box_class() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: box_class =====");
        dbg!();
        let source_string = String::from(
"class Box {}

fun notMethod(argument) {
  print \"called function with \" + argument;
}

var box = Box();
box.function = notMethod;
box.function(\"argument\");
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("called function with argument\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn foo_class() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: foo_class =====");
        dbg!();
        let source_string = String::from(
"class Foo {
  init() {
    print this;
  }
}

var foo = Foo();
print foo.init();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Foo instance\nFoo instance\nFoo instance\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn doughnut() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: doughnut =====");
        dbg!();
        let source_string = String::from(
"class Doughnut {
  cook() {
    print \"Fry until golden brown.\";
  }
}

class BostonCream < Doughnut {}

BostonCream().cook();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Fry until golden brown.\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn boston_cream() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: boston_cream =====");
        dbg!();
        let source_string = String::from(
"class Doughnut {
  cook() {
    print \"Fry until golden brown.\";
  }
}

class BostonCream < Doughnut {
  cook() {
    super.cook();
    print \"Pipe full of custard and coat with chocolate.\";
  }
}

BostonCream().cook();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Fry until golden brown.\nPipe full of custard and coat with chocolate.\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn abc() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: abc =====");
        dbg!();
        let source_string = String::from(
"class A {
  method() {
    print \"A method\";
  }
}

class B < A {
  method() {
    print \"B method\";
  }

  test() {
    super.method();
  }
}

class C < B {}

C().test();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("A method\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn ann() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: ann =====");
        dbg!();
        let source_string = String::from(
"class Person {
  init(name) { this.name = name; }
  greet() { print \"Hi, \" + this.name; }
}
Person(\"Ann\").greet();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("Hi, Ann\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn class_reference() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: class_reference =====");
        dbg!();
        let source_string = String::from(
"class Box {}
var a = Box();
var b = a;
b.x = 1;
print a.x;
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("1\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn triple_inheritance() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: triple_inheritance =====");
        dbg!();
        let source_string = String::from(
"class A { say() { print \"A\"; } }
class B < A { say() { super.say(); print \"B\"; } }
class C < B { say() { super.say(); print \"C\"; } }
C().say();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("A\nB\nC\n", interpreter.result);
        Ok(())
    }

    #[test]
    fn multiple_inheritance() -> Result<(), Box<dyn std::error::Error>> {
        println!("===== test: multiple_inheritance =====");
        dbg!();
        let source_string = String::from(
"class A { say() { print \"A\"; } }
class B < A { say() { super.say(); print \"B\"; } }
class C < B { say() { super.say(); print \"C\"; } }
C().say();
print \"D:\";
class D < A { wow() { super.say(); print \"WOW\"; } }
D().wow();
");
        println!("Input:\n{}=====\nOutput:", source_string);
        let source: Vec<char> = source_string.chars().collect();
        
        let scanner: Scanner = Scanner::new(source);
        let tokens: Vec<Token> = scanner.scan_tokens()?;
        let mut parser: Parser = Parser::new(tokens);
        let expression = parser.parse()?;
        let mut interpreter: Interpreter = Interpreter::new();

        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        resolver.resolve_stmts(&expression);
        if resolver.had_error {
            return Ok(());
        }

        interpreter.interpret(expression)?;

        assert_eq!("A\nB\nC\nD:\nA\nWOW\n", interpreter.result);
        Ok(())
    }
}
