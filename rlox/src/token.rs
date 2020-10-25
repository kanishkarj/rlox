use crate::token_type::TokenType;
use crate::literal::Literal;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line_no: u32,
    pub literal: Option<Literal>,
    pub lexeme: String,
    pub scope: Option<usize>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line_no: u32,
        literal: Option<Literal>,
        lexeme: String,
    ) -> Self {
        Token {
            token_type,
            line_no,
            literal,
            lexeme,
            scope: None,
        }
    }
}

