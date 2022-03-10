use std::io::{self, BufRead, Write};

pub mod token;
pub mod lexer;

mod prelude {
    pub use crate::token::*;
    pub use crate::lexer::*;
}

use prelude::*;


fn main() {
    
    loop {
        print!("Enter something: ");
        // Stdout needs to be flushed, due to missing newline
        io::stdout()
        .flush()
        .expect("Failed to flush stdout");

        let mut line = String::new();
        io::stdin()
        .lock()
        .read_line(&mut line)
        .expect("Something wnt wrong reading line");

        let mut lexer = Lexer::new(&line);

        loop {
            let token = lexer.next_token();
            println!("{:?}", token);
            if token == TokenType::Eof {
                break;
            }
        }
    }
    
}
