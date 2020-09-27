use crate::scanner::*; 
use crate::grammar::LoxCallable; 
use crate::grammar::Expr::*; 
#[derive(Debug, Clone)] 
pub enum Stmt { 
   Expression(Box<Expression>), 
   Block(Box<Block>), 
   Function(Box<Function>), 
   Print(Box<Print>), 
   Var(Box<Var>), 
   While(Box<While>), 
   Break(Box<Break>), 
   Continue(Box<Continue>), 
   If(Box<If>), 
   Return(Box<Return>), 
} 

#[derive(Debug, Clone)] 
pub struct Expression { 
   pub expr: Expr, 
} 

impl Expression { 

        pub fn new(expr: Expr,) -> Self {
            Self {
                expr,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Block { 
   pub statements: Vec<Stmt>, 
} 

impl Block { 

        pub fn new(statements: Vec<Stmt>,) -> Self {
            Self {
                statements,

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

        pub fn new(name: Token,params: Vec<Token>,body: Vec<Stmt>,) -> Self {
            Self {
                name,
params,
body,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Print { 
   pub expr: Expr, 
} 

impl Print { 

        pub fn new(expr: Expr,) -> Self {
            Self {
                expr,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Var { 
   pub name: Token, 
   pub initializer: Option<Expr>, 
} 

impl Var { 

        pub fn new(name: Token,initializer: Option<Expr>,) -> Self {
            Self {
                name,
initializer,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct While { 
   pub condition: Expr, 
   pub body: Stmt, 
} 

impl While { 

        pub fn new(condition: Expr,body: Stmt,) -> Self {
            Self {
                condition,
body,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Break { 
} 

impl Break { 

        pub fn new() -> Self {
            Self {
                
            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Continue { 
} 

impl Continue { 

        pub fn new() -> Self {
            Self {
                
            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct If { 
   pub condition: Expr, 
   pub thenBranch: Stmt, 
   pub elseBranch: Option<Stmt>, 
} 

impl If { 

        pub fn new(condition: Expr,thenBranch: Stmt,elseBranch: Option<Stmt>,) -> Self {
            Self {
                condition,
thenBranch,
elseBranch,

            }
        }

        } 

#[derive(Debug, Clone)] 
pub struct Return { 
   pub keyword: Token, 
   pub value: Option<Expr>, 
} 

impl Return { 

        pub fn new(keyword: Token,value: Option<Expr>,) -> Self {
            Self {
                keyword,
value,

            }
        }

        } 

