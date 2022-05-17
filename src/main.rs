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
    /*
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
    */
        let mut line2 = "
        struct stru 
        {
            double type;
        }

        int poopoo(int t) 
        {
            if(x && y + 1 == true || t || )
            {
                
            }

            int arr[2];
            stru r;
            r = 5;
            s = r.e + (5 / p[1]) - true;
            func(poopoo);

            return 4 + t;
        }
        ";

        let lexer = Lexer::new(&line2);
        let mut analyser = Analyser::new(lexer);
        analyser.start_analysis();
        break;
    }
}
