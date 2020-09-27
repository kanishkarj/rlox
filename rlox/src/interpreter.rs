use crate::grammar::{Expr::*, Stmt::*};
use crate::scanner::*;
use std::fmt::Debug;
use crate::runner::Runner;

use crate::grammar::{Visitor, LoxCallable, LoxFunction, LoxLambda};
use crate::environment::Environment;
use std::time::SystemTime;

#[derive(Debug,Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
    Function(Box<dyn LoxCallable>)
}

pub struct Interpreter {
    pub env: Environment,
}


#[derive(Clone)]
struct ClockFunc;

impl LoxCallable for ClockFunc {
    fn call(&mut self, intr: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> { 
        let curr_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(Object::Num(curr_time.as_millis() as f64))
    }
    fn arity(&mut self) -> usize { 0 }
}

impl Visitor<Object> for Interpreter {
    fn visitBinaryExpr(&mut self, val: &mut Binary) -> Result<Object, LoxError> {
        let mut right = self.evaluate_expr(&mut val.right)?;
        let mut left = self.evaluate_expr(&mut val.left)?;

        match val.operator.tokenType {
            TokenType::MINUS => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left - right))     
                    }
                }
                
                return Err(LoxError::RuntimeError("Operands not Num".to_string(), val.operator.lineNo))
            },
            TokenType::SLASH => {
                if let Object::Num(right) = right {
                    if right == 0.0 {
                        return Err(LoxError::RuntimeError("Division by zero".to_string(), val.operator.lineNo))
                    }
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left / right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Num".to_string(), val.operator.lineNo))
            },
            TokenType::STAR => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left * right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Num".to_string(), val.operator.lineNo))
            },
            TokenType::PLUS => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left + right))        
                    }
                }
                if let Object::Str(right) = right {
                    if let Object::Str(mut left) = left {
                        left.push_str(right.as_str());
                        return Ok(Object::Str(left))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Num or String".to_string(), val.operator.lineNo))
            },
            TokenType::GREATER => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left > right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Num".to_string(), val.operator.lineNo))
            },
            TokenType::GREATER_EQUAL => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left >= right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Num".to_string(), val.operator.lineNo))
            },
            TokenType::LESS => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left < right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Num".to_string(), val.operator.lineNo))
            },
            TokenType::LESS_EQUAL => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left <= right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Num".to_string(), val.operator.lineNo))
            },
            TokenType::BANG_EQUAL => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left != right))        
                    }
                }
                if let Object::Bool(right) = right {
                    if let Object::Bool(left) = left {
                        return Ok(Object::Bool(left != right))        
                    }
                }
                if let Object::Nil = right {
                    if let Object::Nil = left {
                        return Ok(Object::Bool(false))        
                    }
                }
                if let Object::Str(right) = right {
                    if let Object::Str(left) = left {
                        return Ok(Object::Bool(left != right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Comparable".to_string(), val.operator.lineNo))
            },
            TokenType::EQUAL_EQUAL => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left == right))        
                    }
                }
                if let Object::Bool(right) = right {
                    if let Object::Bool(left) = left {
                        return Ok(Object::Bool(left == right))        
                    }
                }
                if let Object::Nil = right {
                    if let Object::Nil = left {
                        return Ok(Object::Bool(true))        
                    }
                }
                if let Object::Str(right) = right {
                    if let Object::Str(left) = left {
                        return Ok(Object::Bool(left == right))        
                    }
                }
                return Err(LoxError::RuntimeError("Operands not Comparable".to_string(), val.operator.lineNo))
            },
            _ => return Err(LoxError::RuntimeError("Operator Unhandled".to_string(), val.operator.lineNo)),
        }
    }

    fn visitGroupingExpr(&mut self, val: &mut Grouping) -> Result<Object, LoxError> { 
        self.evaluate_expr(&mut val.expression)
    }

    fn visitLiteralExpr(&mut self, val: &mut Literal) -> Result<Object, LoxError> { 
        Ok(val.clone().to_object())
    }
    
    fn visitUnaryExpr(&mut self, val: &mut Unary) -> Result<Object, LoxError> { 
        let right = self.evaluate_expr(&mut val.right)?;

        Ok(
            match val.operator.tokenType {
                TokenType::MINUS => {
                    match right {
                        Object::Num(v) => Object::Num(-v),
                        _ => return Err(LoxError::RuntimeError("Unexpected Token found".to_string(), val.operator.lineNo))
                    }
                },
                TokenType::BANG => {
                    Object::Bool(!self.isTrue(&right))
                },
                _ => Object::Nil,
            }
        )
    }
    
    fn visitPrintStmt(&mut self, val: &mut Print) -> std::result::Result<Object, LoxError> { 
        let val = self.evaluate_expr(&mut val.expr)?;
        println!("[print] {:?}", val);
        return Ok(val)
    }
    
    fn visitExpressionStmt(&mut self, val: &mut Expression) -> Result<Object, LoxError> { 
        self.evaluate_expr(&mut val.expr)    
    }

    fn visitVarStmt(&mut self, val: &mut Var) -> Result<Object, LoxError> { 
        let mut value = Object::Nil;
        if let Some(var) = &mut val.initializer {
            value = self.evaluate_expr(var)?;
        }
        self.env.define(val.name.lexeme.clone(), value);
        return Ok(Object::Nil)
     }

    fn visitVariableStmt(&mut self, val: &mut Variable) -> Result<Object, LoxError> {
        // println!("{:?} -> {:?}", val.name.lexeme, self.env.get(val.name.lexeme.clone()));
        self.env.get(val.name.lexeme.clone()).ok_or(
            LoxError::RuntimeError("Undefined get".to_string(), val.name.lineNo)
        )
    }
    fn visitAssignStmt(&mut self, val: &mut Assign) -> Result<Object, LoxError> { 
        let value = self.evaluate_expr(&mut val.value)?;
        if !self.env.assign(val.name.lexeme.clone(), value.clone()) {
            return Err(LoxError::RuntimeError("Undefined assign".to_string(), val.name.lineNo))
        }
        return Ok(value)
     }
    fn visitBlockStmt(&mut self, val: &mut Block) -> Result<Object, LoxError> {
        let env = Environment::from(self.env.clone());
        return self.executeBlock(&mut val.statements, env);
    }

    fn visitIfStmt(&mut self, val: &mut If) -> Result<Object, LoxError> { 
        if let Object::Bool(truthy) = self.evaluate_expr(&mut val.condition)? {
            if truthy {
                self.evaluate_stmt(&mut val.thenBranch)?;
            } else if let Some(stmt) = &mut val.elseBranch {
                self.evaluate_stmt(stmt)?;
            }
        }
        return Ok(Object::Nil);
    }

    fn visitLogicalExpr(&mut self, val: &mut Logical) -> Result<Object, LoxError> { 
        let left = self.evaluate_expr(&mut val.left)?;

        if val.operator.tokenType == TokenType::OR {
            if self.isTrue(&left) {
                return Ok(left)
            }
        } else {
            if !self.isTrue(&left) {
                return Ok(left)
            }
        }

        self.evaluate_expr(&mut val.right)
    }

    fn visitLambdaExpr(&mut self, val: &mut Lambda) -> Result<Object, LoxError> { 
        let func = LoxLambda::new(val.clone(), self.env.clone());
        return Ok(Object::Function(Box::new(func)))
    }

    fn visitWhileStmt(&mut self, val: &mut While) -> Result<Object, LoxError> {
        let mut res = self.evaluate_expr(&mut val.condition)?;
        while self.isTrue(&res) {
            match self.evaluate_stmt(&mut val.body) {
                Err(LoxError::BreakExc) => break,
                Err(LoxError::ContinueExc) => continue,
                _ => {}
            }
            res = self.evaluate_expr(&mut val.condition)?;
        }
        return Ok(Object::Nil);
    }

    fn visitBreakStmt(&mut self, val: &mut Break) -> Result<Object, LoxError> {
        Err(LoxError::BreakExc)
    }
    fn visitContinueStmt(&mut self, val: &mut Continue) -> Result<Object, LoxError> {
        Err(LoxError::ContinueExc)
    }
    
    fn visitCallExpr(&mut self, val: &mut Call) -> Result<Object, LoxError> {
        let callee = self.evaluate_expr(&mut val.callee)?;
        let mut args = Vec::new();
        for arg in &mut val.arguments {
            args.push(self.evaluate_expr(arg)?);
        }

        let mut fn_def: Box<dyn LoxCallable>;

        if let Object::Function(callee) = callee {
            fn_def = callee;
        } else {
            return Err(LoxError::RuntimeError("Not a function".to_string(), val.paren.lineNo))
        }

        if args.len()  != fn_def.arity() {
            return Err(LoxError::RuntimeError("No. of args don't match".to_string(), val.paren.lineNo))
        }
        return fn_def.call(self, args);
    }

    fn visitFunctionStmt(&mut self, val: &mut Function) -> Result<Object, LoxError> {
        let func = LoxFunction::new(val.clone(), self.env.clone());
        self.env.define(val.name.lexeme.clone(), Object::Function(Box::new(func)));
        return Ok(Object::Nil)
    }

    fn visitReturnStmt(&mut self, val: &mut Return) -> Result<Object, LoxError> {
        let mut retValue = Object::Nil;
        if let Some(value) = &mut val.value {
            retValue = self.evaluate_expr(value)?;
        }
        return Err(LoxError::ReturnVal(retValue))
    }
}

impl Interpreter
{
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define("time".to_string(), Object::Function(Box::new(ClockFunc{})));
        Interpreter {
            env
        }
    }

    pub fn executeBlock(&mut self, stmts: &mut Vec<Stmt>, env: Environment) -> Result<Object, LoxError>
        where Self: Visitor<Object>
    {
        let prev = std::mem::replace(&mut self.env, env);
        let mut retVal = Object::Nil;
        for stmt in stmts {
            self.evaluate_stmt(stmt)?;
        }
        self.env = prev;
        Ok(retVal)
    }

    fn isTrue(&mut self, obj: &Object ) -> bool
        where Self: Visitor<Object>
    {
        match obj {
            Object::Bool(v) => return *v,
            Object::Nil => return false,
            _ => return true,
        }
    }


    fn evaluate_expr(&mut self, expr: &mut Expr) -> Result<Object, LoxError>
        where Self: Visitor<Object>
    {
        expr.accept(self)
    }

    fn evaluate_stmt(&mut self, stmt: &mut Stmt) -> Result<Object, LoxError>
        where Self: Visitor<Object>
    {
        stmt.accept(self)
    }

    pub fn interpret(&mut self, statements: &mut Vec<Stmt>)
        where Self: Visitor<Object>, Object: Debug
    {
        for stmt in statements {
            let val = self.evaluate_stmt(stmt);
            match val {
                Err(LoxError::BreakExc) => {Runner::error(0, &"Break".to_string(), &"Break outside loop statement".to_string())},
                Err(LoxError::ContinueExc) => {Runner::error(0, &"Continue".to_string(), &"Continue outside loop statement".to_string())},
                Ok(_) => {},
                Err(err) => err.print_error("Error Intepreting"),
            }
        }
    }

}

// pub fn test() {
//     let mut sample = Expr::Binary(Box::new(Binary::new(
//         Expr::Unary(Box::new(Unary::new(
//             Token::new(TokenType::MINUS, 0, empty_str), 
//             Expr::Literal(Box::new(Literal::new(String::from("123"), LiteralTypes::Num)))
//         ))), 
//         Token::new(TokenType::STAR, 0, empty_str), 
//         Expr::Grouping(Box::new(Grouping::new(
//             Expr::Literal(Box::new(Literal::new(String::from("45.7"), LiteralTypes::Num))))
//         ))
//     )));
//     let mut interpreter = Interpreter{};
//     // println!("{}",sample.accept(&mut Interpreter));
// }