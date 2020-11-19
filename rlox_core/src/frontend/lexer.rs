
use std::error::Error;
use std::fmt::Display;

use logos::Logos;

use crate::runtime::definitions::object::Object;
use crate::frontend::definitions::token_type::TokenType;
use crate::error::LoxError;
use crate::frontend::definitions::literal::Literal;
use crate::frontend::definitions::token::Token;

// TODO: some issue with how we are parsing comments

pub struct Lexer {
    curr: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer { curr: 1 }
    }

    pub fn parse(&mut self, script: &String) -> Result<Vec<Token>, LoxError> {
        let lines = script.split('\n');
        let mut tokens = vec![];
        for line in lines {
            tokens.append(&mut self.read_line(&line.to_string())?);
            self.curr += 1;
        }
        tokens.push(Token::new(TokenType::EOF, self.curr, None, String::new()));
        Ok(tokens)
    }

    pub fn read_line(&mut self, line: &String) -> Result<Vec<Token>, LoxError> {
        let mut tokens = vec![];
        let mut lex = TokenType::lexer(&line);
        //TODO: handle error tokens properly
        while let Some(tk) = lex.next() {
            if tk == TokenType::COMMENTS {
                continue;
            }
            let literal;
            literal = match tk {
                TokenType::TRUE => Some(Literal::BOOL(true)),
                TokenType::FALSE => Some(Literal::BOOL(false)),
                TokenType::STRING => Some(Literal::STRING(
                    lex.slice()[1..(lex.slice().len() - 1)].to_string(),
                )),
                TokenType::NUMBER => Some(Literal::NUM(lex.slice().parse::<f64>()?)),
                _ => None,
            };
            tokens.push(Token::new(tk, self.curr, literal, lex.slice().to_string()))
        }
        return Ok(tokens);
    }
}
