#[derive(Debug)]
#[allow(dead_code)]
pub struct Error {
    pub what: String,
    pub place: String,
    pub line: usize,
}

//pub fn report(err: Error) {
//    println!("[line {}] Error{}: {}", err.line, err.place, err.what);
//    //had_error = true;
//}

pub fn error(err: Error) {
    println!("[line {}] Error: {}", err.line, err.what);
}
