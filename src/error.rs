use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct ScanError {
    pub what: String,
    pub place: String,
    pub line: usize,
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] Error{}: {}", self.line, self.place, self.what)
    }
}

impl Error for ScanError{}
pub fn error(err: ScanError) {
    println!("[line {}] Error: {}", err.line, err.what);
}