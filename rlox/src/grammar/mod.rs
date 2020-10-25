pub mod expr;
pub mod stmt;

use expr::*;
use stmt::*;

use super::scanner::*;
use crate::environment::LocalEnvironment;
use crate::interpreter::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctionType {
    FUNCTION,
    METHOD,
    INITIALIZER,
    NONE,
    LAMBDA,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClassType {
    CLASS,
    SUBCLASS,
    NONE,
}

pub trait VisitorMut<R> {
    fn visit_binary_expr(&mut self, expr: &mut Binary) -> Result<R, LoxError>;
    fn visit_call_expr(&mut self, expr: &mut Call) -> Result<R, LoxError>;
    fn visit_grouping_expr(&mut self, expr: &mut Grouping) -> Result<R, LoxError>;
    fn visit_unary_expr(&mut self, expr: &mut Unary) -> Result<R, LoxError>;
    fn visit_literal_expr(&mut self, expr: &mut Literal) -> Result<R, LoxError>;
    fn visit_logical_expr(&mut self, expr: &mut Logical) -> Result<R, LoxError>;
    fn visit_get_expr(&mut self, expr: &mut Get) -> Result<R, LoxError>;
    fn visit_set_expr(&mut self, expr: &mut Set) -> Result<R, LoxError>;
    fn visit_lambda_expr(&mut self, expr: &mut Lambda) -> Result<R, LoxError>;
    fn visit_this_expr(&mut self, expr: &mut This) -> Result<R, LoxError>;
    fn visit_super_expr(&mut self, expr: &mut Super) -> Result<R, LoxError>;
    fn visit_expression_stmt(&mut self, expr: &mut Expression) -> Result<R, LoxError>;
    fn visit_print_stmt(&mut self, expr: &mut Print) -> Result<R, LoxError>;
    fn visit_variable_stmt(&mut self, expr: &mut Variable) -> Result<R, LoxError>;
    fn visit_var_stmt(&mut self, expr: &mut Var) -> Result<R, LoxError>;
    fn visit_assign_stmt(&mut self, expr: &mut Assign) -> Result<R, LoxError>;
    fn visit_block_stmt(&mut self, expr: &mut Block) -> Result<R, LoxError>;
    fn visit_if_stmt(&mut self, expr: &mut If) -> Result<R, LoxError>;
    fn visit_while_stmt(&mut self, expr: &mut While) -> Result<R, LoxError>;
    fn visit_break_stmt(&mut self, expr: &mut Break) -> Result<R, LoxError>;
    fn visit_continue_stmt(&mut self, expr: &mut Continue) -> Result<R, LoxError>;
    fn visit_function_stmt(&mut self, expr: &mut Function) -> Result<R, LoxError>;
    fn visit_return_stmt(&mut self, expr: &mut Return) -> Result<R, LoxError>;
    fn visit_class_stmt(&mut self, expr: &mut Class) -> Result<R, LoxError>;
}

pub trait VisitorMutAcceptor<T>: Sized {
    fn accept(&mut self, vis: &mut dyn VisitorMut<T>) -> Result<T, LoxError>;
}

impl<T> VisitorMutAcceptor<T> for Expr {
    fn accept(&mut self, vis: &mut dyn VisitorMut<T>) -> Result<T, LoxError> {
        match self {
            Expr::Binary(v) => vis.visit_binary_expr(v),
            Expr::Grouping(v) => vis.visit_grouping_expr(v),
            Expr::Unary(v) => vis.visit_unary_expr(v),
            Expr::Literal(v) => vis.visit_literal_expr(v),
            Expr::Variable(v) => vis.visit_variable_stmt(v),
            Expr::Assign(v) => vis.visit_assign_stmt(v),
            Expr::Logical(v) => vis.visit_logical_expr(v),
            Expr::Call(v) => vis.visit_call_expr(v),
            Expr::Lambda(v) => vis.visit_lambda_expr(v),
            Expr::Get(v) => vis.visit_get_expr(v),
            Expr::Set(v) => vis.visit_set_expr(v),
            Expr::This(v) => vis.visit_this_expr(v),
            Expr::Super(v) => vis.visit_super_expr(v),
        }
    }
}

impl<T> VisitorMutAcceptor<T> for Stmt {
    fn accept(&mut self, vis: &mut dyn VisitorMut<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Expression(v) => vis.visit_expression_stmt(v),
            Stmt::Print(v) => vis.visit_print_stmt(v),
            Stmt::Var(v) => vis.visit_var_stmt(v),
            Stmt::Block(v) => vis.visit_block_stmt(v),
            Stmt::If(v) => vis.visit_if_stmt(v),
            Stmt::While(v) => vis.visit_while_stmt(v),
            Stmt::Break(v) => vis.visit_break_stmt(v),
            Stmt::Continue(v) => vis.visit_continue_stmt(v),
            Stmt::Function(v) => vis.visit_function_stmt(v),
            Stmt::Return(v) => vis.visit_return_stmt(v),
            Stmt::Class(v) => {
                let x = vis.visit_class_stmt(v);
                x
            }
        }
    }
}

impl<T, X> VisitorMutAcceptor<T> for Vec<X>
where
    X: Sized + VisitorMutAcceptor<T>,
    T: Default,
{
    fn accept(&mut self, vis: &mut dyn VisitorMut<T>) -> Result<T, LoxError> {
        for stm in self.iter_mut() {
            stm.accept(vis)?;
        }
        return Ok(T::default());
    }
}

pub trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<R, LoxError>;
    fn visit_call_expr(&mut self, expr: &Call) -> Result<R, LoxError>;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<R, LoxError>;
    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<R, LoxError>;
    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<R, LoxError>;
    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<R, LoxError>;
    fn visit_get_expr(&mut self, expr: &Get) -> Result<R, LoxError>;
    fn visit_set_expr(&mut self, expr: &Set) -> Result<R, LoxError>;
    fn visit_lambda_expr(&mut self, expr: &Lambda) -> Result<R, LoxError>;
    fn visit_this_expr(&mut self, expr: &This) -> Result<R, LoxError>;
    fn visit_super_expr(&mut self, expr: &Super) -> Result<R, LoxError>;
    fn visit_expression_stmt(&mut self, expr: &Expression) -> Result<R, LoxError>;
    fn visit_print_stmt(&mut self, expr: &Print) -> Result<R, LoxError>;
    fn visit_variable_stmt(&mut self, expr: &Variable) -> Result<R, LoxError>;
    fn visit_var_stmt(&mut self, expr: &Var) -> Result<R, LoxError>;
    fn visit_assign_stmt(&mut self, expr: &Assign) -> Result<R, LoxError>;
    fn visit_block_stmt(&mut self, expr: &Block) -> Result<R, LoxError>;
    fn visit_if_stmt(&mut self, expr: &If) -> Result<R, LoxError>;
    fn visit_while_stmt(&mut self, expr: &While) -> Result<R, LoxError>;
    fn visit_break_stmt(&mut self, expr: &Break) -> Result<R, LoxError>;
    fn visit_continue_stmt(&mut self, expr: &Continue) -> Result<R, LoxError>;
    fn visit_function_stmt(&mut self, expr: &Function) -> Result<R, LoxError>;
    fn visit_return_stmt(&mut self, expr: &Return) -> Result<R, LoxError>;
    fn visit_class_stmt(&mut self, expr: &Class) -> Result<R, LoxError>;
}

pub trait VisAcceptor<T>: Sized {
    fn accept(&self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError>;
}

impl<T> VisAcceptor<T> for Expr {
    fn accept(&self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Binary(v) => vis.visit_binary_expr(v),
            Expr::Grouping(v) => vis.visit_grouping_expr(v),
            Expr::Unary(v) => vis.visit_unary_expr(v),
            Expr::Literal(v) => vis.visit_literal_expr(v),
            Expr::Variable(v) => vis.visit_variable_stmt(v),
            Expr::Assign(v) => vis.visit_assign_stmt(v),
            Expr::Logical(v) => vis.visit_logical_expr(v),
            Expr::Call(v) => vis.visit_call_expr(v),
            Expr::Lambda(v) => vis.visit_lambda_expr(v),
            Expr::Get(v) => vis.visit_get_expr(v),
            Expr::Set(v) => vis.visit_set_expr(v),
            Expr::This(v) => vis.visit_this_expr(v),
            Expr::Super(v) => vis.visit_super_expr(v),
        }
    }
}

impl<T> VisAcceptor<T> for Stmt {
    fn accept(&self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Expression(v) => vis.visit_expression_stmt(v),
            Stmt::Print(v) => vis.visit_print_stmt(v),
            Stmt::Var(v) => vis.visit_var_stmt(v),
            Stmt::Block(v) => vis.visit_block_stmt(v),
            Stmt::If(v) => vis.visit_if_stmt(v),
            Stmt::While(v) => vis.visit_while_stmt(v),
            Stmt::Break(v) => vis.visit_break_stmt(v),
            Stmt::Continue(v) => vis.visit_continue_stmt(v),
            Stmt::Function(v) => vis.visit_function_stmt(v),
            Stmt::Return(v) => vis.visit_return_stmt(v),
            Stmt::Class(v) => vis.visit_class_stmt(v),
        }
    }
}

impl<T, X> VisAcceptor<T> for Vec<X>
where
    X: Sized + VisAcceptor<T>,
    T: Default,
{
    fn accept(&self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        for stm in self.iter() {
            stm.accept(vis)?;
        }
        return Ok(T::default());
    }
}

pub trait LoxCallable: LoxCallableClone {
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError>;
    fn arity(&self) -> usize;
}

pub trait LoxCallableClone {
    fn clone_box(&self) -> Box<dyn LoxCallable>;
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

impl std::fmt::Debug for dyn LoxCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "callable")
    }
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: RefCell<Function>,
    closure: LocalEnvironment,
    is_init: bool,
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: LocalEnvironment, is_init: bool) -> Self {
        LoxFunction {
            declaration: RefCell::new(declaration),
            closure,
            is_init,
        }
    }
    pub fn bind(&self, inst: Rc<LoxInstance>) -> Self {
        let env = LocalEnvironment::build(self.closure.clone());
        env.define_at("this".to_string(), Object::Instance(inst), 0);
        LoxFunction::new(self.declaration.borrow().clone(), env, self.is_init)
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let env = LocalEnvironment::build(self.closure.clone());
        for (param, arg) in self.declaration.borrow().params.iter().zip(args) {
            env.define_at(param.lexeme.clone(), arg, 0);
        }
        let val = intrprt.execute_block(&self.declaration.borrow().body, env);
        if let Err(LoxError::ReturnVal(val, _)) = val {
            if self.is_init {
                return Ok(self
                    .closure
                    .get_at("this".to_string(), 0)
                    .unwrap_or(Object::Nil));
            }
            return Ok(val);
        }
        if self.is_init {
            return Ok(self.closure.get_at("this".to_string(), 0).unwrap());
        }
        return val;
    }
    fn arity(&self) -> usize {
        self.declaration.borrow().params.len()
    }
}

#[derive(Debug, Clone)]
pub struct LoxLambda {
    declaration: RefCell<Lambda>,
    closure: LocalEnvironment,
}

impl LoxLambda {
    pub fn new(declaration: Lambda, closure: LocalEnvironment) -> Self {
        LoxLambda {
            declaration: RefCell::new(declaration),
            closure,
        }
    }
}

impl LoxCallable for LoxLambda {
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let env = LocalEnvironment::build(self.closure.clone());
        for (param, arg) in self.declaration.borrow().params.iter().zip(args) {
            env.define_at(param.lexeme.clone(), arg, 0);
        }
        let val = intrprt.execute_block(&self.declaration.borrow_mut().body, env);
        if let Err(LoxError::ReturnVal(val, _)) = val {
            return Ok(val);
        }
        val
    }
    fn arity(&self) -> usize {
        self.declaration.borrow().params.len()
    }
}

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    methods: Rc<HashMap<String, Rc<LoxFunction>>>,
    super_class: Option<Rc<LoxClass>>,
}

impl LoxClass {
    pub fn new(
        name: String,
        methods: Rc<HashMap<String, Rc<LoxFunction>>>,
        super_class: Option<Rc<LoxClass>>,
    ) -> Self {
        LoxClass {
            name,
            methods,
            super_class,
        }
    }
    pub fn find_method(&self, name: &String) -> Option<Rc<LoxFunction>> {
        if let Some(mth) = self.methods.get(name) {
            return Some(mth).cloned();
        } else {
            if let Some(super_class) = &self.super_class {
                return super_class.find_method(name).clone();
            }
        }
        None
    }
    pub fn bind_method(
        &self,
        name: &Token,
        instance: Rc<LoxInstance>,
    ) -> Result<Rc<LoxFunction>, LoxError> {
        if let Some(mth) = self.methods.get(&name.lexeme) {
            return Ok(Rc::new(mth.bind(instance)));
        } else if let Some(super_class) = &self.super_class {
            if let Some(mth) = super_class.find_method(&name.lexeme) {
                return Ok(Rc::new(mth.bind(instance)));
            }
        }

        Err(LoxError::RuntimeError(
            "Only Instances have properties".to_string(),
            name.line_no,
        ))
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let instance = Rc::new(LoxInstance::new(self.clone()));
        if let Some(init) = self.find_method(&"init".to_string()) {
            init.bind(Rc::clone(&instance)).call(intrprt, args)?;
        }
        return Ok(Object::Instance(instance));
    }
    fn arity(&self) -> usize {
        if let Some(init) = self.find_method(&"init".to_string()) {
            init.arity()
        } else {
            0
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub klass: LoxClass,
    fields: RefCell<HashMap<String, Object>>,
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        LoxInstance {
            klass,
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token) -> Option<Object> {
        self.fields.borrow().get(&name.lexeme).cloned()
    }

    pub fn set(&self, name: &Token, value: Object) {
        self.fields.borrow_mut().insert(name.lexeme.clone(), value);
    }
}
