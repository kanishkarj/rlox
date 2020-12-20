use crate::frontend::definitions::expr::*;
use crate::frontend::definitions::stmt::*;
use crate::frontend::lexer::*;
use std::fmt::Debug;

use crate::runtime::environment::{GlobalEnvironment, LocalEnvironment};

use crate::runtime::system_calls::SystemCalls;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use crate::runtime::definitions::lox_callable::LoxCallable;
use crate::runtime::visitor::{VisAcceptor, Visitor};
use crate::runtime::definitions::lox_class::{LoxClass, LoxInstance};
use crate::runtime::definitions::lox_function::{LoxLambda, LoxFunction};
use crate::frontend::definitions::token_type::TokenType;
use crate::error::LoxError;
use crate::frontend::definitions::literal::Literal;
use crate::frontend::definitions::token::Token;
use crate::runtime::definitions::object::Object;

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
    fn get_name(&self) -> String {
        String::from("clock")
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
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num".to_string(),
                ))
            }
            TokenType::SLASH => {
                if let Object::Num(right) = right {
                    if right == 0.0 {
                        return Err(LoxError::RuntimeError(
                            val.operator.lexeme.clone(),
                            val.operator.line_no,
                            "Division by zero".to_string(),
                        ));
                    }
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left / right));
                    }
                }
                Err(LoxError::RuntimeError(
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num".to_string(),
                ))
            }
            TokenType::STAR => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Num(left * right));
                    }
                }
                Err(LoxError::RuntimeError(
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num".to_string(),
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
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num or String".to_string(),
                ))
            }
            TokenType::GREATER => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left > right));
                    }
                }
                Err(LoxError::RuntimeError(
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num".to_string(),
                ))
            }
            TokenType::GreaterEqual => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left >= right));
                    }
                }
                Err(LoxError::RuntimeError(
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num".to_string(),
                ))
            }
            TokenType::LESS => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left < right));
                    }
                }
                Err(LoxError::RuntimeError(
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num".to_string(),
                ))
            }
            TokenType::LessEqual => {
                if let Object::Num(right) = right {
                    if let Object::Num(left) = left {
                        return Ok(Object::Bool(left <= right));
                    }
                }
                Err(LoxError::RuntimeError(
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operands not Num".to_string(),
                ))
            }
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            _ => {
                Err(LoxError::RuntimeError(
                    val.operator.lexeme.clone(),
                    val.operator.line_no,
                    "Operator Unhandled".to_string(),
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
                callee.to_string(),
                val.paren.line_no,
                "Not a function".to_string(),
            ));
        }
        if args.len() != fn_def.arity() {
            return Err(LoxError::RuntimeError(
                fn_def.get_name(),
                val.paren.line_no,
                "No. of args don't match".to_string(),
            ));
        }
        let x = fn_def.call(self, args)?;
        return Ok(x);
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
                        val.operator.lexeme.clone(),
                        val.operator.line_no,
                        "Unexpected Token found".to_string(),
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
            val.name.lexeme.clone(),
            val.name.line_no,
            "Only Instance have properties".to_string(),
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
                val.name.lexeme.clone(),
                val.name.line_no,
                "Only Instances have feilds".to_string(),
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
            val.keyword.lexeme.clone(),
            val.keyword.line_no,
            "Only Instance have properties".to_string(),
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
                val.name.lexeme.clone(),
                val.name.line_no,
                "Undefined assign".to_string(),
            ));
        }

        return Ok(value);
    }

    fn visit_block_stmt(&mut self, val: &Block) -> Result<Object, LoxError> {
        let env = LocalEnvironment::build(self.env.clone());
        return self.execute_block(&val.statements, env);
    }
    fn visit_if_stmt(&mut self, val: &If) -> Result<Object, LoxError> {
        let mut is_true;
        if let Object::Bool(truthy) = self.evaluate(&val.condition)? {
            is_true = truthy;
        } else if let Object::Nil = self.evaluate(&val.condition)? {
            is_true = false;
        } else {
            is_true = true;
        }
                if is_true {
                    self.evaluate(&val.then_branch)?;
                } else if let Some(stmt) = &val.else_branch {
                    self.evaluate(stmt)?;
                }
        return Ok(Object::Nil);
    }

    fn visit_while_stmt(&mut self, val: &While) -> Result<Object, LoxError> {
        let mut res = self.evaluate(&val.condition)?;
        while self.is_true(&res) {
            match self.evaluate(&val.body) {
                Err(LoxError::Break(_)) => break,
                Err(LoxError::Continue(_)) => continue,
                Err(LoxError::ReturnVal(obj,ln)) => return Err(LoxError::ReturnVal(obj,ln)),
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
                    val.name.lexeme.clone(),
                    val.name.line_no,
                    "SuperClass must be a class".to_string(),
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
            println!("{}", method.name.lexeme.clone());
            let func = Rc::new(LoxFunction::new(method.clone(), self.env.clone(), method.name.lexeme == "init"));

            // let func = Rc::new(LoxFunction::new(method.clone(), self.env.clone(), true));
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

    fn visit_stack_trace_stmt(&mut self) -> Result<Object, LoxError> {
        todo!()
    }
}

impl Interpreter {
    pub fn new(syscall: Rc<RefCell<dyn SystemCalls>>) -> Self {
        let env = GlobalEnvironment::new();
        env.define("clock".to_string(), Object::Function(Rc::new(ClockFunc {})));
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
        let err = LoxError::RuntimeError(
            name.lexeme.clone(),
            name.line_no,
            "Undefined get".to_string(),
        );
        return if let Some(dist) = name.scope {
            self.env.get_at(name.lexeme.clone(), dist).ok_or(err.clone())
        } else {
            self.global.get(name.lexeme.clone()).ok_or(err)
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
