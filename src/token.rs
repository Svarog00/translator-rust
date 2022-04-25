#[derive(Debug, PartialEq, Clone)]
pub enum TokenType{
    Illegal,
    Eof,
    
    //Operators
    Function,
    Assign,

    If,
    Else,
    While,
    For,
    Return,

    Plus,
    Minus,
    Multi,
    Divide,
    Equal,
    NotEqual,
    And,
    Or,
    Not,

    Struct,

    Semicolon,
    OpenningBrace,
    ClosingBrace,

    OpenningParenthesis,
    ClosingParenthesis,

    OpenningArray,
    ClosingArray,

    GreaterThan,
    LowerThan,
    GreaterOrEqual,
    LowerOrEqual,

    Dot,
    Comma,

    //Literals
    Identifier(String),
    Type(String),

    //--Types
    Number(String),
    Bool(bool),
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
            "void" => TokenType::Type(ident.to_string()),
            "int" => TokenType::Type(ident.to_string()),
            "double" => TokenType::Type(ident.to_string()),
            "bool" => TokenType::Type(ident.to_string()),
            "while" => TokenType::While,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "return" => TokenType::Return,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            "struct" => TokenType::Struct,
            _ => TokenType::Identifier(ident.to_string()),
        }
    }

    pub fn check_number(number : &str) -> TokenType {
        let bytes = number.as_bytes();
        for &byte in bytes.iter() {
            if !(byte as char).is_numeric() {
                return TokenType::Illegal;
            }
        }
        TokenType::Number(number.to_string())
    }
}