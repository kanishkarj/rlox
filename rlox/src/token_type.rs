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

