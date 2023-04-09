use crate::lexer::tokenize;
use std::io::{self, Write};

pub fn start() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let tokens = tokenize(&line);
        println!("{:?}", tokens);
    }
}
