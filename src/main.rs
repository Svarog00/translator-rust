use std::io::{self, BufRead, Write};

pub mod token;
pub mod lexer;
pub mod syntax_analyser;
pub mod tree;

mod prelude {
    pub use crate::token::*;
    pub use crate::lexer::*;
    pub use crate::syntax_analyser::*;
    pub use crate::tree::*;
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

        let lexer = Lexer::new(&line);
        let mut analyser = Analyser::new(lexer);
        analyser.start_analysis();
    }
    
}
