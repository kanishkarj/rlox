
use std::error::Error;
use std::fmt::Display;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(",")]
    COMMA,

    #[token(".")]
    DOT,

    #[token("-")]
    MINUS,

    #[token("+")]
    PLUS,

    #[token(";")]
    SEMICOLON,

    #[token("/")]
    SLASH,

    #[token("*")]
    STAR,

    // One or two character tokens.
    #[token("!")]
    BANG,

    #[token("!=")]
    BangEqual,

    #[token("=")]
    EQUAL,

    #[token("==")]
    EqualEqual,

    #[token(">")]
    GREATER,

    #[token(">=")]
    GreaterEqual,

    #[token("<")]
    LESS,

    #[token("<=")]
    LessEqual,

    // Keywords.
    #[token("and")]
    AND,

    #[token("class")]
    CLASS,

    #[token("else")]
    ELSE,

    #[token("false")]
    FALSE,

    #[token("fun")]
    FUN,

    #[token("for")]
    FOR,

    #[token("if")]
    IF,

    #[token("nil")]
    NIL,

    #[token("or")]
    OR,

    #[token("print")]
    PRINT,

    #[token("return")]
    RETURN,

    #[token("super")]
    SUPER,

    #[token("this")]
    THIS,

    #[token("true")]
    TRUE,

    #[token("var")]
    VAR,

    #[token("while")]
    WHILE,

    #[token("break")]
    BREAK,

    #[token("continue")]
    CONTINUE,

    // Or regular expressions.
    #[regex("[a-zA-Z]+[a-zA-Z0-9_]*")]
    IDENTIFIER,

    // Or regular expressions.
    #[regex("[0-9]+")]
    NUMBER,

    // Or regular expressions.
    #[regex("\"[^\"]*\"")]
    STRING,

    #[regex("//(?s:[^\"\\\\]|\\\\.)*")]
    COMMENTS,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    EOF,
}

// make return, break into another enum and extend
#[derive(Debug, Clone)]
pub enum LoxError {
    ScannerError(String, u32),
    ParserError(String, u32),
    RuntimeError(String, u32),
    SemanticError(String, u32),
    ReturnVal(Object, u32),
    Break(u32),
    Continue(u32),
}

impl Display for LoxError {
    fn fmt(
        &self,
        writer: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            LoxError::ScannerError(msg, line_no) => {
                writer.write_fmt(format_args!("[ScannerError:L{}] {}", line_no, msg))
            }
            LoxError::ParserError(msg, line_no) => {
                writer.write_fmt(format_args!("[ParserError:L{}] {}", line_no, msg))
            }
            LoxError::RuntimeError(msg, line_no) => {
                writer.write_fmt(format_args!("[RuntimeError:L{}] {}", line_no, msg))
            }
            LoxError::SemanticError(msg, line_no) => {
                writer.write_fmt(format_args!("[SemanticError:L{}] {}", line_no, msg))
            }
            _ => panic!("Handle Types cannot be displayed"),
        }
    }
}

impl std::convert::From<std::time::SystemTimeError> for LoxError {
    fn from(err: std::time::SystemTimeError) -> Self {
        LoxError::RuntimeError(err.to_string().to_string(), 0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NUM(f64),
    STRING(String),
    BOOL(bool),
    NIL,
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

impl Display for Literal {
    fn fmt(
        &self,
        writer: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Literal::NUM(val) => writer.write_str(&val.to_string()),
            Literal::STRING(val) => writer.write_str(&val.to_string()),
            Literal::BOOL(val) => writer.write_str(&val.to_string()),
            Literal::NIL => writer.write_str("Nil"),
        }
    }
}

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
        //TODO: handle error tokens
        while let Some(tk) = lex.next() {
            // println!("{:?}: {}", tk, lex.slice());
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
                TokenType::NUMBER => Some(Literal::NUM(lex.slice().parse::<f64>().unwrap())),
                _ => None,
            };
            tokens.push(Token::new(tk, self.curr, literal, lex.slice().to_string()))
        }
        return Ok(tokens);
    }
}
