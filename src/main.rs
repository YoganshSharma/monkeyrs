mod lexer;
mod ast;
mod parser;
mod repl;
use crate::repl::start;
use std::io::{stdin, stdout, BufReader, BufWriter};
fn main() {
    println!("Welcome to the REPL of monkey lang");
    start(BufReader::new(stdin()), BufWriter::new(stdout())).unwrap();
}
