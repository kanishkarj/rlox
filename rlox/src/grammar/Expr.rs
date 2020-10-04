use crate::scanner::*; 
use crate::grammar::LoxCallable; 
use crate::grammar::Stmt::*;

#[derive(Debug, Clone)] 
pub enum Expr { 
   Binary(Box<Binary>), 
   Grouping(Box<Grouping>), 
   Unary(Box<Unary>), 
   Variable(Box<Variable>), 
   Assign(Box<Assign>), 
   Logical(Box<Logical>), 
   Call(Box<Call>), 
   Lambda(Box<Lambda>), 
   Literal(Literal), 
} 

#[derive(Debug, Clone)] 
pub struct Binary { 
   pub left: Expr, 
   pub operator: Token, 
   pub right: Expr, 
} 

impl Binary { 

        pub fn new(left: Expr,operator: Token,right: Expr,) -> Self {
            Self {
                left,
operator,
right,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Grouping { 
   pub expression: Expr, 
} 

impl Grouping { 

        pub fn new(expression: Expr,) -> Self {
            Self {
                expression,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Unary { 
   pub operator: Token, 
   pub right: Expr, 
} 

impl Unary { 

        pub fn new(operator: Token,right: Expr,) -> Self {
            Self {
                operator,
right,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Variable { 
   pub name: Token, 
} 

impl Variable { 

        pub fn new(name: Token,) -> Self {
            Self {
                name,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Assign { 
   pub name: Token, 
   pub value: Expr, 
} 

impl Assign { 

        pub fn new(name: Token,value: Expr,) -> Self {
            Self {
                name,
value,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Logical { 
   pub left: Expr, 
   pub operator: Token, 
   pub right: Expr, 
} 

impl Logical { 

        pub fn new(left: Expr,operator: Token,right: Expr,) -> Self {
            Self {
                left,
operator,
right,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Call { 
   pub callee: Expr, 
   pub paren: Token, 
   pub arguments: Vec<Expr>, 
} 

impl Call { 

        pub fn new(callee: Expr,paren: Token,arguments: Vec<Expr>,) -> Self {
            Self {
                callee,
paren,
arguments,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Lambda { 
   pub paren: Token, 
   pub params: Vec<Token>, 
   pub body: Vec<Stmt>, 
} 

impl Lambda { 

        pub fn new(paren: Token,params: Vec<Token>,body: Vec<Stmt>,) -> Self {
            Self {
                paren,
params,
body,

            }
        }

        } 
