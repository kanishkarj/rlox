use  scanlex::{Scanner,self};
use std::error::Error;
use crate::runner::Runner;

pub const empty_str: String = String::new();

// make return, break into another enum and extend
#[derive(Debug,Clone)]
pub enum LoxError {
    ScannerError(String, u32),
    ParserError(String, u32),
    RuntimeError(String, u32),
    SemanticError(String, u32),
    ReturnVal(Object),
    BreakExc,
    ContinueExc,
}

impl LoxError {
    pub fn print_error(&self, msg: &str) {
        match self {
            LoxError::ScannerError(lex, line) => Runner::error(*line, lex, &format!("ScannerError: {:?}", msg).to_string()),
            LoxError::ParserError(lex, line) => Runner::error(*line, lex, &format!("ParserError: {:?}", msg).to_string()),
            LoxError::RuntimeError(lex, line) => Runner::error(*line, lex, &format!("RuntimeError: {:?}", msg).to_string()),
            LoxError::SemanticError(lex, line) => Runner::error(*line, lex, &format!("SemanticError: {:?}", msg).to_string()),
            _ => {}
        }
    }
}

impl std::convert::From<std::time::SystemTimeError> for LoxError {
    fn from(err: std::time::SystemTimeError) -> Self { 
        LoxError::RuntimeError(err.description().to_string(), 0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NUM(f64),
    STRING(String),
    BOOL(bool),
    NIL
}

use crate::interpreter::Object;

impl Literal {
    pub fn to_object(self) -> Object {
        match self {
            Literal::NUM(v) => Object::Num(v),
            Literal::STRING(v) => Object::Str(v),
            Literal::BOOL(v) => Object::Bool(v),
            Literal::NIL => Object::Nil,
        }
    }
}


#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum TokenType {

    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
  
    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
  
    // Literals.
    IDENTIFIER, STRING, NUMBER,
  
    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    BREAK, CONTINUE,
  
    // 
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tokenType: TokenType,
    pub lineNo: u32,
    pub literal: Option<Literal>,
    pub lexeme: String,
    pub scope: Option<usize>
}

impl Token {
    pub fn new( tokenType: TokenType,
        lineNo: u32,
        literal: Option<Literal>,
        lexeme: String) -> Self {
            Token {
                tokenType,
                lineNo,
                literal,
                lexeme,
                scope: None
            }
    }
}

// TODO: some issue with how we are parsing comments

pub struct Lexer {
    curr: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            curr: 1,
        }
    }
    
    pub fn parse(&mut self, script: &String) -> Result<Vec<Token>, LoxError> {
        let lines = script.split('\n');
        let mut tokens = vec![];
        for line in lines {
            tokens.append(&mut self.read_line(&line.to_string())?);
            self.curr += 1;
        }
        tokens.push(Token::new(TokenType::EOF,self.curr,None,empty_str));
        Ok(tokens)
    }

    pub fn read_line(&mut self, line: &String) -> Result<Vec<Token>, LoxError> {
        let mut tokens = vec![];
        for token in Scanner::new(line) {
            match token {
                scanlex::Token::Char(v) => {
                    match v {
                        '+' => tokens.push(Token::new(TokenType::PLUS,self.curr,None,empty_str)),
                        '(' => tokens.push(Token::new(TokenType::LEFT_PAREN,self.curr,None,empty_str)),
                        ')' => tokens.push(Token::new(TokenType::RIGHT_PAREN,self.curr,None,empty_str)),
                        '{' => tokens.push(Token::new(TokenType::LEFT_BRACE,self.curr,None,empty_str)),
                        '}' => tokens.push(Token::new(TokenType::RIGHT_BRACE,self.curr,None,empty_str)),
                        ',' => tokens.push(Token::new(TokenType::COMMA,self.curr,None,empty_str)),
                        '.' => tokens.push(Token::new(TokenType::DOT,self.curr,None,empty_str)),
                        '-' => tokens.push(Token::new(TokenType::MINUS,self.curr,None,empty_str)),
                        '+' => tokens.push(Token::new(TokenType::PLUS,self.curr,None,empty_str)),
                        '*' => tokens.push(Token::new(TokenType::STAR,self.curr,None,empty_str)),
                        // Prev value dependents
                        '/' => {
                            let cache = tokens.pop();
                            if cache.is_none() {
                                tokens.push(Token::new(TokenType::SLASH,self.curr,None,empty_str));
                            } else if cache.unwrap().tokenType == TokenType::SLASH
                            {
                                return Ok(vec![])
                            }
                        }
                        '=' => {
                            let cache = tokens.pop();
                            if cache.is_none() {
                                tokens.push(Token::new(TokenType::EQUAL,self.curr,None,empty_str));
                            } else {
                                let cache = cache.unwrap();
                                if cache.tokenType == TokenType::EQUAL {
                                    tokens.push(Token::new(TokenType::EQUAL_EQUAL,self.curr,None,empty_str))
                                } else if cache.tokenType == TokenType::BANG {
                                    tokens.push(Token::new(TokenType::BANG_EQUAL,self.curr,None,empty_str))
                                } else if cache.tokenType == TokenType::LESS {
                                    tokens.push(Token::new(TokenType::LESS_EQUAL,self.curr,None,empty_str))
                                } else if cache.tokenType == TokenType::GREATER {
                                    tokens.push(Token::new(TokenType::GREATER_EQUAL,self.curr,None,empty_str))
                                } else {
                                    tokens.push(cache);
                                    tokens.push(Token::new(TokenType::EQUAL,self.curr,None,empty_str));
                                }
                            }
                        }
                        '>' => tokens.push(Token::new(TokenType::GREATER,self.curr,None,empty_str)),
                        '<' => tokens.push(Token::new(TokenType::LESS,self.curr,None,empty_str)),
                        '!' => tokens.push(Token::new(TokenType::BANG,self.curr,None,empty_str)),
                        ';' => tokens.push(Token::new(TokenType::SEMICOLON,self.curr,None,empty_str)),
                        _ => {
                            return Err(LoxError::ScannerError(format!("Unknown character: {}", v), self.curr))
                        }
                    }
                },
                scanlex::Token::Iden(v) => {
                    match v.as_str() {
                        "and" => tokens.push(Token::new(TokenType::AND,self.curr,None,empty_str)),
                        "class" => tokens.push(Token::new(TokenType::CLASS,self.curr,None,empty_str)),
                        "else" => tokens.push(Token::new(TokenType::ELSE,self.curr,None,empty_str)),
                        "false" => tokens.push(Token::new(TokenType::FALSE,self.curr,None,empty_str)),
                        "for" => tokens.push(Token::new(TokenType::FOR,self.curr,None,empty_str)),
                        "fun" => tokens.push(Token::new(TokenType::FUN,self.curr,None,empty_str)),
                        "if" => tokens.push(Token::new(TokenType::IF,self.curr,None,empty_str)),
                        "nil" => tokens.push(Token::new(TokenType::NIL,self.curr,None,empty_str)),
                        "or" => tokens.push(Token::new(TokenType::OR,self.curr,None,empty_str)),
                        "print" => tokens.push(Token::new(TokenType::PRINT,self.curr,None,empty_str)),
                        "return" => tokens.push(Token::new(TokenType::RETURN,self.curr,None,empty_str)),
                        "super" => tokens.push(Token::new(TokenType::SUPER,self.curr,None,empty_str)),
                        "this" => tokens.push(Token::new(TokenType::THIS,self.curr,None,empty_str)),
                        "true" => tokens.push(Token::new(TokenType::TRUE,self.curr,None,empty_str)),
                        "var" => tokens.push(Token::new(TokenType::VAR,self.curr,None,empty_str)),
                        "while" => tokens.push(Token::new(TokenType::WHILE,self.curr,None,empty_str)),
                        "break" => tokens.push(Token::new(TokenType::BREAK,self.curr,None,empty_str)),
                        "continue" => tokens.push(Token::new(TokenType::CONTINUE,self.curr,None,empty_str)),
                        _ => tokens.push(Token::new(TokenType::IDENTIFIER,self.curr,None,v))
                    }
                },
                scanlex::Token::Str(v) => {tokens.push(Token::new(TokenType::STRING,self.curr,Some(Literal::STRING(v.clone())), v))},
                scanlex::Token::Int(v) => {tokens.push(Token::new(TokenType::NUMBER,self.curr,Some(Literal::NUM(v as f64)), v.to_string()))},
                scanlex::Token::Num(v) => {tokens.push(Token::new(TokenType::NUMBER,self.curr,Some(Literal::NUM(v)), v.to_string()))},
                scanlex::Token::End => {tokens.push(Token::new(TokenType::EOF,self.curr,None,empty_str))},
                scanlex::Token::Error(v) => {
                    return Err(LoxError::ScannerError(v.description().to_string(), self.curr))
                },
            }
        }
        return Ok(tokens);
    }
}