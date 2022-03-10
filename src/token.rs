#[derive(Debug, PartialEq)]
pub enum TokenType{
    Illegal,
    Eof,
    
    //Operators
    Function,
    Let,
    Equal,

    //Literals
    Identifier (String),

    //--Types
    Integer (String),
    Double(String),
}

impl Default for TokenType{
    fn default() -> TokenType{
        TokenType::Illegal
    }
}

impl TokenType {
    pub fn check_ident(ident : &str) -> TokenType {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            _ => TokenType::Identifier(ident.to_string()),
        }
    }
}