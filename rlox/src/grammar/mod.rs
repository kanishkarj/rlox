pub mod Expr;
pub mod Stmt;

use Expr::*;
use Stmt::*;

use super::scanner::Literal;
use crate::scanner::LoxError;
use crate::interpreter::*;
use crate::environment::Environment;

pub trait Visitor<R> { 
    fn visitBinaryExpr (&mut self, expr: &mut Binary) -> Result<R, LoxError>; 
    fn visitCallExpr (&mut self, expr: &mut Call) -> Result<R, LoxError>; 
    fn visitGroupingExpr (&mut self, expr: &mut Grouping) -> Result<R, LoxError>; 
    fn visitUnaryExpr (&mut self, expr: &mut Unary) -> Result<R, LoxError>; 
    fn visitLiteralExpr (&mut self, expr: &mut Literal) -> Result<R, LoxError>;
    fn visitLogicalExpr (&mut self, expr: &mut Logical) -> Result<R, LoxError>;
    fn visitLambdaExpr (&mut self, expr: &mut Lambda) -> Result<R, LoxError>;
    fn visitExpressionStmt (&mut self, expr: &mut Expression) -> Result<R, LoxError>;
    fn visitPrintStmt (&mut self, expr: &mut Print) -> Result<R, LoxError>;
    fn visitVariableStmt (&mut self, expr: &mut Variable) -> Result<R, LoxError>;
    fn visitVarStmt (&mut self, expr: &mut Var) -> Result<R, LoxError>;
    fn visitAssignStmt (&mut self, expr: &mut Assign) -> Result<R, LoxError>;
    fn visitBlockStmt (&mut self, expr: &mut Block) -> Result<R, LoxError>;
    fn visitIfStmt (&mut self, expr: &mut If) -> Result<R, LoxError>;
    fn visitWhileStmt (&mut self, expr: &mut While) -> Result<R, LoxError>;
    fn visitBreakStmt (&mut self, expr: &mut Break) -> Result<R, LoxError>;
    fn visitContinueStmt (&mut self, expr: &mut Continue) -> Result<R, LoxError>;
    fn visitFunctionStmt (&mut self, expr: &mut Function) -> Result<R, LoxError>;
    fn visitReturnStmt (&mut self, expr: &mut Return) -> Result<R, LoxError>;
 }

impl Expr::Expr {
    pub fn accept<'a, T> (&mut self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Expr::Binary(v) => {vis.visitBinaryExpr(v)}, 
            Expr::Expr::Grouping(v) => {vis.visitGroupingExpr(v)}, 
            Expr::Expr::Unary(v) => {vis.visitUnaryExpr(v)}, 
            Expr::Expr::Literal(v) => vis.visitLiteralExpr(v),
            Expr::Expr::Variable(v) => vis.visitVariableStmt(v),
            Expr::Expr::Assign(v) => vis.visitAssignStmt(v),
            Expr::Expr::Logical(v) => vis.visitLogicalExpr(v),
            Expr::Expr::Call(v) => vis.visitCallExpr(v),
            Expr::Expr::Lambda(v) => vis.visitLambdaExpr(v),
        }
        }
    }

    impl Stmt::Stmt {
        pub fn accept<'a, T> (&mut self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError> {
            match self {
                Stmt::Stmt::Expression(v) => {vis.visitExpressionStmt(v)}, 
                Stmt::Stmt::Print(v) => {vis.visitPrintStmt(v)},
                Stmt::Stmt::Var(v) => vis.visitVarStmt(v),
                Stmt::Stmt::Block(v) => vis.visitBlockStmt(v),
                Stmt::Stmt::If(v) => vis.visitIfStmt(v),
                Stmt::Stmt::While(v) => {vis.visitWhileStmt(v)},
                Stmt::Stmt::Break(v) =>{vis.visitBreakStmt(v)},
                Stmt::Stmt::Continue(v) => vis.visitContinueStmt(v),
                Stmt::Stmt::Function(v) => vis.visitFunctionStmt(v),
                Stmt::Stmt::Return(v) => vis.visitReturnStmt(v),
            }
            }
        }
        pub trait LoxCallable: LoxCallableClone {
            fn call(&mut self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError>;
            fn arity(&mut self) -> usize;
        }
        
        trait LoxCallableClone {
            fn clone_box(&self) -> Box<LoxCallable>;
        }
        
        impl<T> LoxCallableClone for T
        where
            T: 'static + LoxCallable + Clone,
        {
            fn clone_box(&self) -> Box<dyn LoxCallable> {
                Box::new(self.clone())
            }
        }
        
        // We can now implement Clone manually by forwarding to clone_box.
        impl Clone for Box<dyn LoxCallable> {
            fn clone(&self) -> Box<dyn LoxCallable> {
                self.clone_box()
            }
        }

        impl std::fmt::Debug for LoxCallable {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", "callable")
            }
        }

        #[derive(Debug, Clone)]
        pub struct LoxFunction {
            declaration: Function,
            closure: Environment,
        }

        impl LoxFunction {
            pub fn new(declaration: Function, closure: Environment) -> Self {
                LoxFunction {
                    declaration,
                    closure
                }
            }
        }

        impl LoxCallable for LoxFunction {
            
                fn arity(&mut self) -> usize { self.declaration.params.len()}
                fn call(&mut self, intrprt: &mut Interpreter, args:Vec<Object>) -> Result<Object, LoxError> { 
                    let mut env = Environment::form(self.closure.clone());
                    for (param,arg) in self.declaration.params.iter().zip(args) {
                        env.define(param.lexeme.clone(), arg);
                    }
                    let val = intrprt.executeBlock(&mut self.declaration.body, env);
                    if let Err(LoxError::ReturnVal(val)) = val {
                        return Ok(val)
                    }
                    return val
                }
        }

        #[derive(Debug, Clone)]
        pub struct LoxLambda {
            declaration: Lambda,
            closure: Environment,
        }

        impl LoxLambda {
            pub fn new(declaration: Lambda, closure: Environment) -> Self {
                LoxLambda {
                    declaration,
                    closure
                }
            }
        }

        impl LoxCallable for LoxLambda {
            
                fn arity(&mut self) -> usize { self.declaration.params.len()}
                fn call(&mut self, intrprt: &mut Interpreter, args:Vec<Object>) -> Result<Object, LoxError> { 
                    let mut env = Environment::form(self.closure.clone());
                    // intrprt.env.create_scope();
                    for (param,arg) in self.declaration.params.iter().zip(args) {
                        env.define(param.lexeme.clone(), arg);
                    }
                    let val = intrprt.executeBlock(&mut self.declaration.body, env)?;
                    Ok(val)
                }
        } 