use crate::grammar::{expr::*, stmt::*};
use crate::scanner::*;
use std::fmt::Debug;

use crate::environment::{GlobalEnvironment, LocalEnvironment};

use crate::system_calls::SystemCalls;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use crate::grammar::lox_callable::LoxCallable;
use crate::grammar::visitor::{VisAcceptor, Visitor};
use crate::grammar::lox_class::{LoxClass, LoxInstance};
use crate::grammar::lox_function::{LoxLambda, LoxFunction};
use crate::token_type::TokenType;
use crate::error::LoxError;
use crate::literal::Literal;
use crate::token::Token;

// obj.get not handled
#[derive(Debug, Clone)]
pub enum Object {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
    Function(Rc<dyn LoxCallable>),
    Class(Rc<LoxClass>),
    Instance(Rc<LoxInstance>),
}

impl From<Literal> for Object {
    fn from(val: Literal) -> Self {
        match val {
            Literal::NUM(v) => Object::Num(v),
            Literal::STRING(v) => Object::Str(v),
            Literal::BOOL(v) => Object::Bool(v),
            Literal::NIL => Object::Nil,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        use Object::*;

        match (self, other) {
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Num(ref a), &Num(ref b)) => a == b,
            (&Bool(ref a), &Bool(ref b)) => a == b,
            (&Nil, &Nil) => true,
            _ => false,
        }
    }
}

impl Display for Object {
    fn fmt(
        &self,
        writer: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Object::Str(val) => writer.write_str(&val.to_string()),
            Object::Num(val) => writer.write_str(&val.to_string()),
            Object::Bool(val) => writer.write_str(&val.to_string()),
            Object::Nil => writer.write_str("Nil"),
            Object::Function(_val) => writer.write_str("Function"),
            Object::Class(val) => writer.write_str(&val.name),
            Object::Instance(val) => {
                writer.write_fmt(format_args!("Instance<{}>", &val.klass.name))
            }
        }
    }
}

pub struct Interpreter {
    pub env: LocalEnvironment,
    pub global: GlobalEnvironment,
    system_interface: Rc<RefCell<dyn SystemCalls>>,
}

#[derive(Clone)]
struct ClockFunc;

impl LoxCallable for ClockFunc {
    fn call(&self, intr: &mut Interpreter, _args: Vec<Object>) -> Result<Object, LoxError> {
        intr.system_interface.borrow_mut().time()
    }
    fn arity(&self) -> usize {
        0
    }
}

impl Visitor<Object> for Interpreter {
    fn visit_binary_expr(&mut self, val: &Binary) -> Result<Object, LoxError> {
        let right = self.evaluate(&val.right)?;
        let left = self.evaluate(&val.left)?;

        return match val.operator.token_type {
            //TODO: TRy do these stuff with traits
            TokenType::MINUS => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left - right));
                    }
                }

                Err(LoxError::RuntimeError(
                    "Operands not Num".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::SLASH => {
                if let Object::Num(right) = right {
                    if right == 0.0 {
                        return Err(LoxError::RuntimeError(
                            "Division by zero".to_string(),
                            val.operator.line_no,
                        ));
                    }
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left / right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Num".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::STAR => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left * right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Num".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::PLUS => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left + right));
                    }
                }
                if let Object::Str(right) = right {
                    if let Object::Str(mut left) = left {
                        left.push_str(right.as_str());
                        return Ok(Object::Str(left));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Num or String".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::GREATER => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left > right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Num".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::GreaterEqual => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left >= right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Num".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::LESS => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left < right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Num".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::LessEqual => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left <= right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Num".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::BangEqual => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left != right));
                    }
                }
                if let Object::Bool(right) = right {
                    if let Object::Bool(left) = left {
                        return Ok(Object::Bool(left != right));
                    }
                }
                if let Object::Nil = right {
                    if let Object::Nil = left {
                        return Ok(Object::Bool(false));
                    }
                }
                if let Object::Str(right) = right {
                    if let Object::Str(left) = left {
                        return Ok(Object::Bool(left != right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Comparable".to_string(),
                    val.operator.line_no,
                ))
            }
            TokenType::EqualEqual => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left == right));
                    }
                }
                if let Object::Bool(right) = right {
                    if let Object::Bool(left) = left {
                        return Ok(Object::Bool(left == right));
                    }
                }
                if let Object::Nil = right {
                    if let Object::Nil = left {
                        return Ok(Object::Bool(true));
                    }
                }
                if let Object::Str(right) = right {
                    if let Object::Str(left) = left {
                        return Ok(Object::Bool(left == right));
                    }
                }
                Err(LoxError::RuntimeError(
                    "Operands not Comparable".to_string(),
                    val.operator.line_no,
                ))
            }
            _ => {
                Err(LoxError::RuntimeError(
                    "Operator Unhandled".to_string(),
                    val.operator.line_no,
                ))
            }
        }
    }

    fn visit_call_expr(&mut self, val: &Call) -> Result<Object, LoxError> {
        let callee = self.evaluate(&val.callee)?;
        let mut args = Vec::new();
        for arg in &val.arguments {
            args.push(self.evaluate(arg)?);
        }

        let fn_def: Rc<dyn LoxCallable>;

        if let Object::Function(callee) = callee {
            fn_def = callee;
        } else if let Object::Class(callee) = callee {
            fn_def = callee;
        } else {
            return Err(LoxError::RuntimeError(
                "Not a function".to_string(),
                val.paren.line_no,
            ));
        }

        if args.len() != fn_def.arity() {
            return Err(LoxError::RuntimeError(
                "No. of args don't match".to_string(),
                val.paren.line_no,
            ));
        }
        let x = fn_def.call(self, args);
        return x;
    }

    fn visit_grouping_expr(&mut self, val: &Grouping) -> Result<Object, LoxError> {
        self.evaluate(&val.expression)
    }

    fn visit_unary_expr(&mut self, val: &Unary) -> Result<Object, LoxError> {
        let right = self.evaluate(&val.right)?;

        Ok(match val.operator.token_type {
            TokenType::MINUS => match right {
                Object::Num(v) => Object::Num(-v),
                _ => {
                    return Err(LoxError::RuntimeError(
                        "Unexpected Token found".to_string(),
                        val.operator.line_no,
                    ))
                }
            },
            TokenType::BANG => Object::Bool(!self.is_true(&right)),
            _ => Object::Nil,
        })
    }

    fn visit_literal_expr(&mut self, val: &Literal) -> Result<Object, LoxError> {
        Ok(val.clone().into())
    }

    fn visit_logical_expr(&mut self, val: &Logical) -> Result<Object, LoxError> {
        let left = self.evaluate(&val.left)?;

        if val.operator.token_type == TokenType::OR {
            if self.is_true(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_true(&left) {
                return Ok(left);
            }
        }

        self.evaluate(&val.right)
    }

    fn visit_get_expr(&mut self, val: &Get) -> Result<Object, LoxError> {
        let obj = self.evaluate(&val.object)?;
        if let Object::Instance(inst) = obj {
            return if let Some(val) = inst.get(&val.name) {
                Ok(val)
            } else {
                Ok(Object::Function(
                    inst.klass.bind_method(&val.name, Rc::clone(&inst))?,
                ))
            }
        }
        Err(LoxError::RuntimeError(
            "Only Instance have properties".to_string(),
            val.name.line_no,
        ))
    }

    fn visit_set_expr(&mut self, val: &Set) -> Result<Object, LoxError> {
        let mut obj = self.evaluate(&val.object)?;
        return if let Object::Instance(obj) = &mut obj {
            let value = self.evaluate(&val.value)?;
            obj.set(&val.name, value.clone());
            Ok(value)
        } else {
            Err(LoxError::RuntimeError(
                "Only Instances have feilds".to_string(),
                val.name.line_no,
            ))
        }
    }

    fn visit_lambda_expr(&mut self, val: &Lambda) -> Result<Object, LoxError> {
        let func = LoxLambda::new(val.clone(), self.env.clone());
        return Ok(Object::Function(Rc::new(func)));
    }

    fn visit_this_expr(&mut self, val: &This) -> Result<Object, LoxError> {
        self.variable_lookup(&val.keyword)
    }

    fn visit_super_expr(&mut self, val: &Super) -> Result<Object, LoxError> {
        let err = LoxError::RuntimeError(
            "Only Instance have properties".to_string(),
            val.keyword.line_no,
        );
        if let Some(dist) = val.keyword.scope {
            let super_class = self
                .env
                .get_at("super".to_string(), dist)
                .ok_or(err.clone())?;
            let this_obj = self
                .env
                .get_at("this".to_string(), dist - 1)
                .ok_or(err.clone())?;

            if let Object::Class(super_class) = super_class {
                if let Object::Instance(this_obj) = this_obj {
                    if let Some(method) = super_class.find_method(&val.method.lexeme) {
                        return Ok(Object::Function(Rc::new(method.bind(Rc::clone(&this_obj)))));
                    }
                }
            }
        }
        Err(err)
    }

    fn visit_expression_stmt(&mut self, val: &Expression) -> Result<Object, LoxError> {
        self.evaluate(&val.expr)
    }

    fn visit_print_stmt(&mut self, val: &Print) -> std::result::Result<Object, LoxError> {
        let res = self.evaluate(&val.expr)?;
        self.system_interface.borrow_mut().print(&res);
        return Ok(res);
    }

    fn visit_variable_stmt(&mut self, val: &Variable) -> Result<Object, LoxError> {
        self.variable_lookup(&val.name)
    }

    fn visit_var_stmt(&mut self, val: &Var) -> Result<Object, LoxError> {
        let mut value = Object::Nil;
        if let Some(var) = &val.initializer {
            value = self.evaluate(var)?;
        }
        if let Some(dist) = val.name.scope {
            self.env.define_at(val.name.lexeme.clone(), value, dist);
        } else {
            self.global.define(val.name.lexeme.clone(), value);
        }
        return Ok(Object::Nil);
    }

    fn visit_assign_stmt(&mut self, val: &Assign) -> Result<Object, LoxError> {
        let value = self.evaluate(&val.value)?;
        if !(if let Some(dist) = val.name.scope {
            self.env
                .assign_at(val.name.lexeme.clone(), value.clone(), dist)
        } else {
            self.global.assign(val.name.lexeme.clone(), value.clone())
        }) {
            return Err(LoxError::RuntimeError(
                "Undefined assign".to_string(),
                val.name.line_no,
            ));
        }

        return Ok(value);
    }

    fn visit_block_stmt(&mut self, val: &Block) -> Result<Object, LoxError> {
        let env = LocalEnvironment::build(self.env.clone());
        return self.execute_block(&val.statements, env);
    }
    fn visit_if_stmt(&mut self, val: &If) -> Result<Object, LoxError> {
        if let Object::Bool(truthy) = self.evaluate(&val.condition)? {
            if truthy {
                self.evaluate(&val.then_branch)?;
            } else if let Some(stmt) = &val.else_branch {
                self.evaluate(stmt)?;
            }
        }
        return Ok(Object::Nil);
    }

    fn visit_while_stmt(&mut self, val: &While) -> Result<Object, LoxError> {
        let mut res = self.evaluate(&val.condition)?;
        while self.is_true(&res) {
            match self.evaluate(&val.body) {
                Err(LoxError::Break(_)) => break,
                Err(LoxError::Continue(_)) => continue,
                _ => {}
            }
            res = self.evaluate(&val.condition)?;
        }
        return Ok(Object::Nil);
    }

    fn visit_break_stmt(&mut self, val: &Break) -> Result<Object, LoxError> {
        Err(LoxError::Break(val.keyword.line_no))
    }

    fn visit_continue_stmt(&mut self, val: &Continue) -> Result<Object, LoxError> {
        Err(LoxError::Continue(val.keyword.line_no))
    }

    fn visit_function_stmt(&mut self, val: &Function) -> Result<Object, LoxError> {
        let func = LoxFunction::new(val.clone(), self.env.clone(), false);
        self.env
            .define_at(val.name.lexeme.clone(), Object::Function(Rc::new(func)), 0);
        return Ok(Object::Nil);
    }

    fn visit_return_stmt(&mut self, val: &Return) -> Result<Object, LoxError> {
        let mut ret_value = Object::Nil;
        if let Some(value) = &val.value {
            ret_value = self.evaluate(value)?;
        }
        return Err(LoxError::ReturnVal(ret_value, val.keyword.line_no));
    }

    fn visit_class_stmt(&mut self, val: &Class) -> Result<Object, LoxError> {
        let mut super_class = None;
        if let Some(sp_class) = &val.superclass {
            if let Object::Class(value) = &self.visit_variable_stmt(sp_class)? {
                super_class = Some(Rc::clone(value));
                self.env = LocalEnvironment::build(self.env.clone());
                self.env
                    .define_at("super".to_string(), Object::Class(Rc::clone(value)), 0);
            } else {
                return Err(LoxError::RuntimeError(
                    "SuperClass must be a class".to_string(),
                    val.name.line_no,
                ));
            }
        }

        if let Some(hops) = val.name.scope {
            self.env
                .define_at(val.name.lexeme.clone(), Object::Nil, hops);
        } else {
            self.global.define(val.name.lexeme.clone(), Object::Nil);
        }

        let mut methods = HashMap::new();

        for method in &val.methods {
            let func = Rc::new(LoxFunction::new(method.clone(), self.env.clone(), true));
            methods.insert(method.name.lexeme.clone(), func);
        }

        let klass = Object::Class(Rc::new(LoxClass::new(
            val.name.lexeme.clone(),
            Rc::new(methods),
            super_class,
        )));

        if let Some(hops) = val.name.scope {
            self.env.assign_at(val.name.lexeme.clone(), klass, hops);
        } else {
            self.global.assign(val.name.lexeme.clone(), klass);
        }
        return Ok(Object::Nil);
    }
}

impl Interpreter {
    pub fn new(syscall: Rc<RefCell<dyn SystemCalls>>) -> Self {
        let env = GlobalEnvironment::new();
        env.define("time".to_string(), Object::Function(Rc::new(ClockFunc {})));
        Interpreter {
            env: LocalEnvironment::from(env.clone()),
            global: env,
            system_interface: syscall,
        }
    }

    pub fn execute_block(
        &mut self,
        stmts: &Vec<Stmt>,
        env: LocalEnvironment,
    ) -> Result<Object, LoxError>
    where
        Self: Visitor<Object>,
    {
        let prev = std::mem::replace(&mut self.env, env);
        let ret_val = Object::Nil;
        for stmt in stmts {
            match self.evaluate(stmt) {
                Ok(_) => {}
                err => {
                    self.env = prev;
                    return err;
                }
            };
        }
        self.env = prev;
        Ok(ret_val)
    }

    fn is_true(&self, obj: &Object) -> bool
    where
        Self: Visitor<Object>,
    {
        return match obj {
            Object::Bool(v) => *v,
            Object::Nil => false,
            _ => true,
        }
    }

    fn variable_lookup(&mut self, name: &Token) -> Result<Object, LoxError>
    where
        Self: Visitor<Object>,
    {
        let err = LoxError::RuntimeError("Undefined get: ".to_string() + &name.lexeme, name.line_no);
        return if let Some(dist) = name.scope {
            self.env.get_at(name.lexeme.clone(), dist).ok_or(err)
        } else {
            self.global.get(name.lexeme.clone()).ok_or(err.clone())
        };
    }

    fn evaluate<T: VisAcceptor<Object> + Sized>(&mut self, expr: &T) -> Result<Object, LoxError>
    where
        Self: Visitor<Object>,
    {
        expr.accept(self)
    }

    pub fn interpret(&mut self, statements: &mut Vec<Stmt>) -> Result<(), LoxError>
    where
        Self: Visitor<Object>,
        Object: Debug,
    {
        for stmt in statements {
            let val = self.evaluate(stmt);
            match val {
                Ok(_) => {}
                Err(err) => return Err(err),
            }
        }
        Ok(())
    }
}
