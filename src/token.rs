#[derive(Debug, PartialEq)]
pub enum TokenType{
    Illegal,
    Eof,
    
    //Operators
    Function,
    Let,
    Assign,

    If,
    While,
    For,

    Plus,
    Minus,
    Equal,
    NotEqual,

    Semicolon,
    OpenningBrace,
    ClosingBrace,

    OpenningParenthesis,
    ClosingParenthesis,

    GreaterThan,
    LowerThan,

    GreaterOrEqual,
    LowerOrEqual,

    //Literals
    Identifier (String),

    //--Types
    Integer(String),
    Double(String),
}

impl Default for TokenType{
    fn default() -> TokenType{
        TokenType::Illegal
    }
}

impl TokenType {
    pub fn check_identifier(ident : &str) -> TokenType {
        match ident {
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "while" => TokenType::While,
            "if" => TokenType::If,
            "for" => TokenType::For,
            _ => TokenType::Identifier(ident.to_string()),
        }
    }

    pub fn check_number(number : &str) -> TokenType {
        let bytes = number.as_bytes();
        for &byte  in bytes.iter() {
            if (byte as char).is_alphabetic() {
                return TokenType::Illegal;
            }
        }
        TokenType::Integer(number.to_string())
    }
}