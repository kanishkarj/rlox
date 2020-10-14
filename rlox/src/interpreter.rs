use crate::grammar::{Expr::*, Stmt::*};
use crate::scanner::*;
use std::fmt::Debug;
use crate::runner::Runner;

use crate::grammar::{Visitor, LoxCallable, LoxFunction, LoxLambda, LoxClass, LoxInstance, VisAcceptor};
use crate::environment::Environment;
use std::time::SystemTime;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Display;
use std::collections::HashMap;

// obj.get not handled

#[derive(Debug,Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
    Function(Rc<RefCell<dyn LoxCallable>>),
    Class(Rc<RefCell<LoxClass>>),
    Instance(Rc<RefCell<LoxInstance>>),
}

impl Display for Object {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Str(val) => writer.write_str(&val.to_string()),
            Object::Num(val) => writer.write_str(&val.to_string()), 
            Object::Bool(val) => writer.write_str(&val.to_string()), 
            Object::Nil => writer.write_str("Nil"),
            Object::Function(val) => writer.write_str("Function"), 
            Object::Class(val) => writer.write_str(&val.borrow().name), 
            Object::Instance(val) => writer.write_fmt(format_args!("Instance<{}>", &val.borrow().klass.name)), 
        }
    }
}

pub struct Interpreter {
    pub env: Environment,
    pub global: Environment,
}

#[derive(Clone)]
struct ClockFunc;

impl LoxCallable for ClockFunc {
    fn call(&mut self, intr: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> { 
        let curr_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(Object::Num(curr_time.as_millis() as f64))
    }
    fn arity(&self) -> usize { 0 }
}

impl Visitor<Object> for Interpreter {
    fn visitBinaryExpr(&mut self, val: &mut Binary) -> Result<Object, LoxError> {
        let mut right = self.evaluate(&mut val.right)?;
        let mut left = self.evaluate(&mut val.left)?;

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
        self.evaluate(&mut val.expression)
    }

    fn visitLiteralExpr(&mut self, val: &mut Literal) -> Result<Object, LoxError> { 
        Ok(val.clone().to_object())
    }
    
    fn visitUnaryExpr(&mut self, val: &mut Unary) -> Result<Object, LoxError> { 
        let right = self.evaluate(&mut val.right)?;

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
    
    fn visitGetExpr(&mut self, val: &mut Get) -> Result<Object, LoxError> { 
        
        let obj = self.evaluate(&mut val.object)?;
        if let Object::Instance(inst) = obj {
            if let Some(val) = inst.borrow().get(&val.name) {
                return Ok(val);
            } else {
                return Ok(Object::Function(
                    inst.borrow().klass.bind_method(&val.name, Rc::clone(&inst))?
                ));
            }
        }
        Err(LoxError::RuntimeError("Only Instance have properties".to_string(), val.name.lineNo))
    }

    fn visitSetExpr(&mut self, val: &mut Set) -> Result<Object, LoxError> {
        let mut obj = self.evaluate(&mut val.object)?;
        if let Object::Instance(obj) = &mut obj {
            let value = self.evaluate(&mut val.value)?;
            obj.borrow().set(&val.name, value.clone());
            return Ok(value)
        } else {
            return Err(LoxError::RuntimeError("Only Instances have feilds".to_string(), val.name.lineNo))
        }
    }

    fn visitPrintStmt(&mut self, val: &mut Print) -> std::result::Result<Object, LoxError> { 
        let val = self.evaluate(&mut val.expr)?;
        println!("[print] {}", val);
        return Ok(val)
    }
    
    fn visitExpressionStmt(&mut self, val: &mut Expression) -> Result<Object, LoxError> { 
        self.evaluate(&mut val.expr)    
    }

    fn visitVarStmt(&mut self, val: &mut Var) -> Result<Object, LoxError> { 
        let mut value = Object::Nil;
        if let Some(var) = &mut val.initializer {
            value = self.evaluate(var)?;
        }
        if let Some(dist) = val.name.scope {
            self.env.defineAt(val.name.lexeme.clone(), value, dist);
        } else {
            self.global.define(val.name.lexeme.clone(), value);
        }
        return Ok(Object::Nil)
     }

    fn visitVariableStmt(&mut self, val: &mut Variable) -> Result<Object, LoxError> {
        self.variableLookup(&mut val.name)
    }
    fn visitAssignStmt(&mut self, val: &mut Assign) -> Result<Object, LoxError> { 
        let value = self.evaluate(&mut val.value)?;
        if !(if let Some(dist) = val.name.scope {
            self.env.assignAt(val.name.lexeme.clone(), value.clone(), dist)
        } else {
            self.global.assign(val.name.lexeme.clone(), value.clone())
        }) {
            return Err(LoxError::RuntimeError("Undefined assign".to_string(), val.name.lineNo))
        }
        
        return Ok(value)
     }
    fn visitBlockStmt(&mut self, val: &mut Block) -> Result<Object, LoxError> {
        let env = Environment::build(self.env.clone());
        return self.executeBlock(&mut val.statements, env);
    }

    fn visitIfStmt(&mut self, val: &mut If) -> Result<Object, LoxError> { 
        if let Object::Bool(truthy) = self.evaluate(&mut val.condition)? {
            if truthy {
                self.evaluate(&mut val.thenBranch)?;
            } else if let Some(stmt) = &mut val.elseBranch {
                self.evaluate(stmt)?;
            }
        }
        return Ok(Object::Nil);
    }

    fn visitLogicalExpr(&mut self, val: &mut Logical) -> Result<Object, LoxError> { 
        let left = self.evaluate(&mut val.left)?;

        if val.operator.tokenType == TokenType::OR {
            if self.isTrue(&left) {
                return Ok(left)
            }
        } else {
            if !self.isTrue(&left) {
                return Ok(left)
            }
        }

        self.evaluate(&mut val.right)
    }

    fn visitLambdaExpr(&mut self, val: &mut Lambda) -> Result<Object, LoxError> { 
        let func = LoxLambda::new(val.clone(), self.env.clone());
        return Ok(Object::Function(Rc::new(RefCell::new(func))))
    }

    fn visitWhileStmt(&mut self, val: &mut While) -> Result<Object, LoxError> {
        let mut res = self.evaluate(&mut val.condition)?;
        while self.isTrue(&res) {
            match self.evaluate(&mut val.body) {
                Err(LoxError::BreakExc(_)) => break,
                Err(LoxError::ContinueExc(_)) => continue,
                _ => {}
            }
            res = self.evaluate(&mut val.condition)?;
        }
        return Ok(Object::Nil);
    }

    fn visitBreakStmt(&mut self, val: &mut Break) -> Result<Object, LoxError> {
        Err(LoxError::BreakExc(val.keyword.lineNo))
    }
    fn visitContinueStmt(&mut self, val: &mut Continue) -> Result<Object, LoxError> {
        Err(LoxError::ContinueExc(val.keyword.lineNo))
    }
    
    fn visitCallExpr(&mut self, val: &mut Call) -> Result<Object, LoxError> {
        let callee = self.evaluate(&mut val.callee)?;
        let mut args = Vec::new();
        for arg in &mut val.arguments {
            args.push(self.evaluate(arg)?);
        }

        let mut fn_def: Rc<RefCell<dyn LoxCallable>>;

        if let Object::Function(callee) = callee {
            fn_def = callee;
        } else if let Object::Class(callee) = callee {
            fn_def = callee;
        } else {
            return Err(LoxError::RuntimeError("Not a function".to_string(), val.paren.lineNo))
        }

        if args.len()  != fn_def.borrow().arity() {
            return Err(LoxError::RuntimeError("No. of args don't match".to_string(), val.paren.lineNo))
        }
        return fn_def.borrow_mut().call(self, args);
    }

    fn visitThisExpr (&mut self, val: &mut This) -> Result<Object, LoxError> {
        self.variableLookup(&mut val.keyword)
    }

    fn visitFunctionStmt(&mut self, val: &mut Function) -> Result<Object, LoxError> {
        let func = LoxFunction::new(val.clone(), self.env.clone(), false);
        self.env.define(val.name.lexeme.clone(), Object::Function(Rc::new(RefCell::new(func))));
        return Ok(Object::Nil)
    }

    fn visitReturnStmt(&mut self, val: &mut Return) -> Result<Object, LoxError> {
        let mut retValue = Object::Nil;
        if let Some(value) = &mut val.value {
            retValue = self.evaluate(value)?;
        }
        return Err(LoxError::ReturnVal(retValue))
    }

    fn visitClassStmt(&mut self, val: &mut Class) -> Result<Object, LoxError> {
        let mut superClass = None;
        if let Some(spClass) = &mut val.superclass {
            if let Object::Class(value) = &self.visitVariableStmt(spClass)?{
                superClass = Some(Rc::clone(value));
                self.env = Environment::build(self.env.clone());
                self.env.define("super".to_string(), Object::Class(Rc::clone(value)))
            } 
            else {
                return Err(LoxError::RuntimeError("SuperClass must be a class".to_string(), val.name.lineNo));
            }
        }
        self.env.define(val.name.lexeme.clone(), Object::Nil);

        let mut methods = HashMap::new();

        for method in &val.methods {
            let func = Rc::new(LoxFunction::new(method.clone(), self.env.clone(), true));
            methods.insert(method.name.lexeme.clone(), func);
        }

        let klass = Object::Class(Rc::new(RefCell::new(LoxClass::new(val.name.lexeme.clone(), Rc::new(methods), superClass))));
        self.env.assign(val.name.lexeme.clone(), klass);
        return Ok(Object::Nil);
    }

    fn visitSuperExpr(&mut self, val: &mut Super) -> Result<Object, LoxError> {
        let err = LoxError::RuntimeError("Only Instance have properties".to_string(), val.keyword.lineNo);
        if let Some(dist) = val.keyword.scope {
            let superClass = self.env.getAt("super".to_string(), dist).ok_or(err.clone())?;
            let thisObj = self.env.getAt("this".to_string(), dist-1).ok_or(err.clone())?;
            
            if let Object::Class(superClass) = superClass {
                if let Object::Instance(thisObj) = thisObj {
                    if let Some(method) = superClass.borrow().findMethod(&val.method.lexeme) {
                        return Ok(Object::Function(Rc::new(RefCell::new(method.bind(Rc::clone(&thisObj))))))
                    }
                }
            }
        }
        Err(err)
    }
}

impl Interpreter
{
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define("time".to_string(), Object::Function(Rc::new(RefCell::new(ClockFunc{}))));
        Interpreter {
            env:            env.clone(),
            global: env
        }
    }

    pub fn executeBlock(&mut self, stmts: &mut Vec<Stmt>, env: Environment) -> Result<Object, LoxError>
        where Self: Visitor<Object>
    {
        let prev = std::mem::replace(&mut self.env, env);
        let mut retVal = Object::Nil;
        for stmt in stmts {
            match self.evaluate(stmt) {
                Ok(_) => {},
                err => {
                    self.env = prev;
                    return err
                }
            };
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

    fn variableLookup(&mut self, name: &mut Token) -> Result<Object, LoxError>
        where Self: Visitor<Object>
    {
        let err = LoxError::RuntimeError("Undefined get".to_string(), name.lineNo);
        return if let Some(dist) = name.scope {
            self.env.getAt(name.lexeme.clone(), dist).ok_or(err)
        } else {
            self.global.get(name.lexeme.clone()).ok_or(err)
        }
        
    }

    fn evaluate<T: VisAcceptor<Object> + Sized> (&mut self, expr: &mut T) -> Result<Object, LoxError>
        where Self: Visitor<Object>
    {
        expr.accept(self)
    }

    pub fn interpret(&mut self, statements: &mut Vec<Stmt>)
        where Self: Visitor<Object>, Object: Debug
    {
        for stmt in statements {
            let val = self.evaluate(stmt);
            match val {
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