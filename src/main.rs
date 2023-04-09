mod lexer;
mod repl;
mod token;

use crate::repl::start;

fn main() {
    println!("Hello,! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    start();
}
