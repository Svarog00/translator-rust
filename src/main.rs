pub mod token;
pub mod lexer;
pub mod syntax_analyser;
pub mod tree;
pub mod tree_structure;

mod prelude {
    pub use crate::token::*;
    pub use crate::lexer::*;
    pub use crate::syntax_analyser::*;
    pub use crate::tree::*;
    pub use crate::tree_structure::*;
}

use prelude::*;


fn main() {
    
    loop {
        let mut line2 = "
        struct stru 
        {
            double g;
        }

        int poopoo(int t, double a) 
        {
            while(x && y + 1 == true || t || w && e)
            {
                r = 1;
            }

            int arr[2];
            stru r = 1;
            r = 5;
            s = r.e + (5 / p[1]) - true;
            func(poopoo,2);
        }
        ";

        let lexer = Lexer::new(&line2);
        let mut analyser = Analyser::new(lexer);
        let mut ast_tree = AstTree::new(analyser.start_analysis(), analyser.struct_types);
        ast_tree.write_out();
        break;
    }
}
