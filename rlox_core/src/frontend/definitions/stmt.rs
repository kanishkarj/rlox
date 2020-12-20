use crate::frontend::definitions::expr::*;
use crate::frontend::lexer::*;
use crate::frontend::definitions::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Box<Expression>),
    Block(Box<Block>),
    Class(Box<Class>),
    Function(Box<Function>),
    Print(Box<Print>),
    Var(Box<Var>),
    While(Box<While>),
    Break(Box<Break>),
    Continue(Box<Continue>),
    If(Box<If>),
    Return(Box<Return>),
    StackTrace,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expr: Expr,
}

impl Expression {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Token,
    pub methods: Vec<Function>,
    pub superclass: Option<Variable>,
}

impl Class {
    pub fn new(name: Token, methods: Vec<Function>, superclass: Option<Variable>) -> Self {
        Self {
            name,
            methods,
            superclass,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self { name, params, body }
    }
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expr: Expr,
    pub token: Token,
}

impl Print {
    pub fn new(expr: Expr, token: Token) -> Self {
        Self { expr, token }
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

impl Var {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Self { name, initializer }
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub token: Token,
    pub condition: Expr,
    pub body: Stmt,
}

impl While {
    pub fn new(condition: Expr, body: Stmt, token: Token) -> Self {
        Self { token, condition, body }
    }
}

#[derive(Debug, Clone)]
pub struct Break {
    pub keyword: Token,
}

impl Break {
    pub fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}

#[derive(Debug, Clone)]
pub struct Continue {
    pub keyword: Token,
}

impl Continue {
    pub fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub token: Token,
    pub condition: Expr,
    pub then_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

impl If {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>, token: Token) -> Self {
        Self {
            token,
            condition,
            then_branch,
            else_branch,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    pub keyword: Token,
    pub value: Option<Expr>,
}

impl Return {
    pub fn new(keyword: Token, value: Option<Expr>) -> Self {
        Self { keyword, value }
    }
}
