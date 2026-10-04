# rs-lox

Lox interpreter in Rust by Dylan Tejada.

## Project Structure

The src/ directory contains the source files for the project

The target/ directory contains files rust uses for building and running

Tests are located in the associated files, so tests for the scanner are in scanner.rs

Tests have already been run and their output stored in testing.log

## Installation and Running

Make sure you have Rust and Cargo installed.

Run with:

    $ cargo run --bin rs-lox [source-file]

Running without a source file will start the REPL, which you can exit with CTRL+D.

To run all tests:

    $ cargo test

To see test output:

    $ cargo test -- --show-output

For chapter 5 ast printer run:

    $ cargo run --bin astprinter