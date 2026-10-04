# rs-lox

Lox interpreter in Rust.

## Installation and Running

Make sure you have Rust and Cargo installed.

Run with:

    $ cargo run --bin rs-lox [source-file]

Running without a source file will start the REPL, which you can exit with CTRL+D.

For chapter 5 ast printer run:

    $ cargo run --bin astprinter

To run all tests:

    $ cargo test

To see output from tests:

    $ cargo test -- --show-output
