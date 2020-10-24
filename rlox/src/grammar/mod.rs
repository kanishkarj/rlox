pub mod Expr;
pub mod Stmt;

use Expr::*;
use Stmt::*;

use super::scanner::*;
use crate::environment::{LocalEnvironment};
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
    LAMBDA
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ClassType {
    CLASS,
    SUBCLASS,
    NONE,
}

pub trait VisitorMut<R> {
    fn visitBinaryExpr(&mut self, expr: &mut Binary) -> Result<R, LoxError>;
    fn visitCallExpr(&mut self, expr: &mut Call) -> Result<R, LoxError>;
    fn visitGroupingExpr(&mut self, expr: &mut Grouping) -> Result<R, LoxError>;
    fn visitUnaryExpr(&mut self, expr: &mut Unary) -> Result<R, LoxError>;
    fn visitLiteralExpr(&mut self, expr: &mut Literal) -> Result<R, LoxError>;
    fn visitLogicalExpr(&mut self, expr: &mut Logical) -> Result<R, LoxError>;
    fn visitGetExpr(&mut self, expr: &mut Get) -> Result<R, LoxError>;
    fn visitSetExpr(&mut self, expr: &mut Set) -> Result<R, LoxError>;
    fn visitLambdaExpr(&mut self, expr: &mut Lambda) -> Result<R, LoxError>;
    fn visitThisExpr(&mut self, expr: &mut This) -> Result<R, LoxError>;
    fn visitSuperExpr(&mut self, expr: &mut Super) -> Result<R, LoxError>;
    fn visitExpressionStmt(&mut self, expr: &mut Expression) -> Result<R, LoxError>;
    fn visitPrintStmt(&mut self, expr: &mut Print) -> Result<R, LoxError>;
    fn visitVariableStmt(&mut self, expr: &mut Variable) -> Result<R, LoxError>;
    fn visitVarStmt(&mut self, expr: &mut Var) -> Result<R, LoxError>;
    fn visitAssignStmt(&mut self, expr: &mut Assign) -> Result<R, LoxError>;
    fn visitBlockStmt(&mut self, expr: &mut Block) -> Result<R, LoxError>;
    fn visitIfStmt(&mut self, expr: &mut If) -> Result<R, LoxError>;
    fn visitWhileStmt(&mut self, expr: &mut While) -> Result<R, LoxError>;
    fn visitBreakStmt(&mut self, expr: &mut Break) -> Result<R, LoxError>;
    fn visitContinueStmt(&mut self, expr: &mut Continue) -> Result<R, LoxError>;
    fn visitFunctionStmt(&mut self, expr: &mut Function) -> Result<R, LoxError>;
    fn visitReturnStmt(&mut self, expr: &mut Return) -> Result<R, LoxError>;
    fn visitClassStmt(&mut self, expr: &mut Class) -> Result<R, LoxError>;
}

pub trait VisitorMutAcceptor<T>: Sized {
    fn accept(&mut self, vis: &mut dyn VisitorMut<T>) -> Result<T, LoxError>;
}

impl<T> VisitorMutAcceptor<T> for Expr::Expr {
    fn accept(&mut self, vis: &mut dyn VisitorMut<T>) -> Result<T, LoxError> {
        match self {
            Expr::Expr::Binary(v) => vis.visitBinaryExpr(v),
            Expr::Expr::Grouping(v) => vis.visitGroupingExpr(v),
            Expr::Expr::Unary(v) => vis.visitUnaryExpr(v),
            Expr::Expr::Literal(v) => vis.visitLiteralExpr(v),
            Expr::Expr::Variable(v) => vis.visitVariableStmt(v),
            Expr::Expr::Assign(v) => vis.visitAssignStmt(v),
            Expr::Expr::Logical(v) => vis.visitLogicalExpr(v),
            Expr::Expr::Call(v) => vis.visitCallExpr(v),
            Expr::Expr::Lambda(v) => vis.visitLambdaExpr(v),
            Expr::Expr::Get(v) => vis.visitGetExpr(v),
            Expr::Expr::Set(v) => vis.visitSetExpr(v),
            Expr::Expr::This(v) => vis.visitThisExpr(v),
            Expr::Expr::Super(v) => vis.visitSuperExpr(v),
        }
    }
}

impl<T> VisitorMutAcceptor<T> for Stmt::Stmt {
    fn accept(&mut self, vis: &mut dyn VisitorMut<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Stmt::Expression(v) => vis.visitExpressionStmt(v),
            Stmt::Stmt::Print(v) => vis.visitPrintStmt(v),
            Stmt::Stmt::Var(v) => vis.visitVarStmt(v),
            Stmt::Stmt::Block(v) => vis.visitBlockStmt(v),
            Stmt::Stmt::If(v) => vis.visitIfStmt(v),
            Stmt::Stmt::While(v) => vis.visitWhileStmt(v),
            Stmt::Stmt::Break(v) => vis.visitBreakStmt(v),
            Stmt::Stmt::Continue(v) => vis.visitContinueStmt(v),
            Stmt::Stmt::Function(v) => vis.visitFunctionStmt(v),
            Stmt::Stmt::Return(v) => vis.visitReturnStmt(v),
            Stmt::Stmt::Class(v) => {
                let x = vis.visitClassStmt(v);
                x
            },
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
    fn visitBinaryExpr(&mut self, expr: &Binary) -> Result<R, LoxError>;
    fn visitCallExpr(&mut self, expr: &Call) -> Result<R, LoxError>;
    fn visitGroupingExpr(&mut self, expr: &Grouping) -> Result<R, LoxError>;
    fn visitUnaryExpr(&mut self, expr: &Unary) -> Result<R, LoxError>;
    fn visitLiteralExpr(&mut self, expr: &Literal) -> Result<R, LoxError>;
    fn visitLogicalExpr(&mut self, expr: &Logical) -> Result<R, LoxError>;
    fn visitGetExpr(&mut self, expr: &Get) -> Result<R, LoxError>;
    fn visitSetExpr(&mut self, expr: &Set) -> Result<R, LoxError>;
    fn visitLambdaExpr(&mut self, expr: &Lambda) -> Result<R, LoxError>;
    fn visitThisExpr(&mut self, expr: &This) -> Result<R, LoxError>;
    fn visitSuperExpr(&mut self, expr: &Super) -> Result<R, LoxError>;
    fn visitExpressionStmt(&mut self, expr: &Expression) -> Result<R, LoxError>;
    fn visitPrintStmt(&mut self, expr: &Print) -> Result<R, LoxError>;
    fn visitVariableStmt(&mut self, expr: &Variable) -> Result<R, LoxError>;
    fn visitVarStmt(&mut self, expr: &Var) -> Result<R, LoxError>;
    fn visitAssignStmt(&mut self, expr: &Assign) -> Result<R, LoxError>;
    fn visitBlockStmt(&mut self, expr: &Block) -> Result<R, LoxError>;
    fn visitIfStmt(&mut self, expr: &If) -> Result<R, LoxError>;
    fn visitWhileStmt(&mut self, expr: &While) -> Result<R, LoxError>;
    fn visitBreakStmt(&mut self, expr: &Break) -> Result<R, LoxError>;
    fn visitContinueStmt(&mut self, expr: &Continue) -> Result<R, LoxError>;
    fn visitFunctionStmt(&mut self, expr: &Function) -> Result<R, LoxError>;
    fn visitReturnStmt(&mut self, expr: &Return) -> Result<R, LoxError>;
    fn visitClassStmt(&mut self, expr: &Class) -> Result<R, LoxError>;
}

pub trait VisAcceptor<T>: Sized {
    fn accept(&self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError>;
}

impl<T> VisAcceptor<T> for Expr::Expr {
    fn accept(&self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Expr::Expr::Binary(v) => vis.visitBinaryExpr(v),
            Expr::Expr::Grouping(v) => vis.visitGroupingExpr(v),
            Expr::Expr::Unary(v) => vis.visitUnaryExpr(v),
            Expr::Expr::Literal(v) => vis.visitLiteralExpr(v),
            Expr::Expr::Variable(v) => vis.visitVariableStmt(v),
            Expr::Expr::Assign(v) => vis.visitAssignStmt(v),
            Expr::Expr::Logical(v) => vis.visitLogicalExpr(v),
            Expr::Expr::Call(v) => vis.visitCallExpr(v),
            Expr::Expr::Lambda(v) => vis.visitLambdaExpr(v),
            Expr::Expr::Get(v) => vis.visitGetExpr(v),
            Expr::Expr::Set(v) => vis.visitSetExpr(v),
            Expr::Expr::This(v) => vis.visitThisExpr(v),
            Expr::Expr::Super(v) => vis.visitSuperExpr(v),
        }
    }
}

impl<T> VisAcceptor<T> for Stmt::Stmt {
    fn accept(&self, vis: &mut dyn Visitor<T>) -> Result<T, LoxError> {
        match self {
            Stmt::Stmt::Expression(v) => vis.visitExpressionStmt(v),
            Stmt::Stmt::Print(v) => vis.visitPrintStmt(v),
            Stmt::Stmt::Var(v) => vis.visitVarStmt(v),
            Stmt::Stmt::Block(v) => vis.visitBlockStmt(v),
            Stmt::Stmt::If(v) => vis.visitIfStmt(v),
            Stmt::Stmt::While(v) => vis.visitWhileStmt(v),
            Stmt::Stmt::Break(v) => vis.visitBreakStmt(v),
            Stmt::Stmt::Continue(v) => vis.visitContinueStmt(v),
            Stmt::Stmt::Function(v) => vis.visitFunctionStmt(v),
            Stmt::Stmt::Return(v) => vis.visitReturnStmt(v),
            Stmt::Stmt::Class(v) => vis.visitClassStmt(v),
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
    declaration: RefCell<Function>,
    closure: LocalEnvironment,
    isInit: bool,
}

impl LoxFunction {
    pub fn new(declaration: Function, closure: LocalEnvironment, isInit: bool) -> Self {
        LoxFunction {
            declaration: RefCell::new(declaration),
            closure,
            isInit,
        }
    }
    pub fn bind(&self, inst: Rc<LoxInstance>) -> Self {
        let mut env = LocalEnvironment::build(self.closure.clone());
        env.defineAt("this".to_string(), Object::Instance(inst), 0);
        LoxFunction::new(self.declaration.borrow().clone(), env, self.isInit)
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.declaration.borrow().params.len()
    }
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let mut env = LocalEnvironment::build(self.closure.clone());
        for (param, arg) in self.declaration.borrow().params.iter().zip(args) {
            env.defineAt(param.lexeme.clone(), arg, 0);
        }
        let val = intrprt.executeBlock(&self.declaration.borrow().body, env);
        if let Err(LoxError::ReturnVal(val,_)) = val {
            if self.isInit {
                return Ok(self
                    .closure
                    .getAt("this".to_string(), 0)
                    .unwrap_or(Object::Nil));
            }
            return Ok(val);
        }
        if self.isInit {
            return Ok(self.closure.getAt("this".to_string(), 0).unwrap());
        }
        return val;
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
    fn arity(&self) -> usize {
        self.declaration.borrow().params.len()
    }
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let mut env = LocalEnvironment::build(self.closure.clone());
        for (param, arg) in self.declaration.borrow().params.iter().zip(args) {
            env.defineAt(param.lexeme.clone(), arg, 0);
        }
        let val = intrprt.executeBlock(&self.declaration.borrow_mut().body, env);
        if let Err(LoxError::ReturnVal(val,_)) = val {
            return Ok(val);
        }
        val
    }
}

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    methods: Rc<HashMap<String, Rc<LoxFunction>>>,
    superClass: Option<Rc<LoxClass>>,
}

impl LoxClass {
    pub fn new(
        name: String,
        methods: Rc<HashMap<String, Rc<LoxFunction>>>,
        superClass: Option<Rc<LoxClass>>,
    ) -> Self {
        LoxClass {
            name,
            methods,
            superClass,
        }
    }
    pub fn findMethod(&self, name: &String) -> Option<Rc<LoxFunction>> {
        if let Some(mth) = self.methods.get(name) {
            return Some(mth).cloned();
        } else {
            if let Some(superClass) = &self.superClass {
                let x = superClass.findMethod(name);
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
        } else if let Some(superClass) = &self.superClass {
            if let Some(mth) = superClass.findMethod(&name.lexeme) {
                return Ok(Rc::new(mth.bind(instance)));
            }
        }

        Err(LoxError::RuntimeError(
            "Only Instances have properties".to_string(),
            name.lineNo,
        ))
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        if let Some(init) = self.findMethod(&"init".to_string()) {
            init.arity()
        } else {
            0
        }
    }
    fn call(&self, intrprt: &mut Interpreter, args: Vec<Object>) -> Result<Object, LoxError> {
        let instance = Rc::new(LoxInstance::new(self.clone()));
        if let Some(init) = self.findMethod(&"init".to_string()) {
            init.bind(Rc::clone(&instance)).call(intrprt, args)?;
        }
        return Ok(Object::Instance(instance));
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
