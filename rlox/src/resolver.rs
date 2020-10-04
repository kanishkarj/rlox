use crate::interpreter::Interpreter;
use crate::grammar::{Expr::*, Stmt::*};
use crate::scanner::*;
use std::fmt::Debug;
use crate::runner::Runner;

use crate::grammar::{Visitor, LoxCallable, LoxFunction, LoxLambda};
use crate::environment::Environment;

use std::collections::HashMap;

pub struct Resolver {
    // bool corresponds to if the value has been initialized
    pub scopes: Vec<HashMap<String,bool>>
}

impl Resolver {
    pub fn new() -> Self {
        Resolver{
            scopes: vec![]
        }
    }

    pub fn resolve(&mut self, stmts: &mut Vec<Stmt>)  -> Result<(), LoxError> {
        for stm in stmts {
            self.resolve_st(stm)?;
        }
        Ok(())
    }

    fn resolve_st(&mut self, stmt: &mut Stmt)  -> Result<(), LoxError> {
        stmt.accept(self)?;
        Ok(())
    }

    fn resolve_exp(&mut self, exp: &mut Expr)  -> Result<(), LoxError> {
        exp.accept(self)?;
        Ok(())
    }

    fn resolveLocal(&mut self, name: &mut Token) {
        for (i,scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                name.scope = Some(self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn beginScope(&mut self){
        self.scopes.push(HashMap::new());
    }
    
    fn endScope(&mut self){
        self.scopes.pop();
    }
    
    fn declare(&mut self, token: &Token){
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(token.lexeme.clone(), false);
        }
    }

    fn define(&mut self, token: &Token) -> Result<(), LoxError> {
        if let Some(scope) = self.scopes.last_mut(){
            scope.insert(token.lexeme.clone(), true).ok_or(LoxError::SemanticError("Declaration without definition".to_string(), token.lineNo))?;
        }
        Ok(())
    }

    fn resolveFunction(&mut self, func: &mut Function) -> Result<(), LoxError>  {
        self.beginScope();
        for param in &func.params {
            self.declare(&param);
            self.define(&param)?;
        }
        self.resolve(&mut func.body)?;
        self.endScope();
        Ok(())
    }

    fn resolveLambda(&mut self, func: &mut Lambda) -> Result<(), LoxError>  {
        self.beginScope();
        for param in &func.params {
            self.declare(&param);
            self.define(&param)?;
        }
        self.resolve(&mut func.body)?;
        self.endScope();
        Ok(())
    }
}


impl Visitor<()> for Resolver {

    fn visitBinaryExpr (&mut self, val: &mut Binary) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.left)?;
        self.resolve_exp(&mut val.right)?;
        Ok(())
    }

    fn visitCallExpr (&mut self, val: &mut Call) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.callee)?;
        for arg in &mut val.arguments {
            self.resolve_exp(arg)?;
        }
        Ok(())
    }

    fn visitGroupingExpr (&mut self, val: &mut Grouping) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.expression)?;
        Ok(())
    }

    fn visitUnaryExpr (&mut self, val: &mut Unary) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.right)?;
        Ok(())
    }

    fn visitLiteralExpr (&mut self, val: &mut Literal) -> Result<(), LoxError> {
        Ok(())
    }

    fn visitLogicalExpr (&mut self, val: &mut Logical) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.left)?;
        self.resolve_exp(&mut val.right)?;
        Ok(())
    }

    fn visitLambdaExpr (&mut self, val: &mut Lambda) -> Result<(), LoxError> {
        self.resolveLambda(val)?;
        Ok(())
    }

    fn visitExpressionStmt (&mut self, val: &mut Expression) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.expr)?;
        Ok(())
    }

    fn visitPrintStmt (&mut self, val: &mut Print) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.expr)?;
        Ok(())
    }

    fn visitVariableStmt (&mut self, val: &mut Variable) -> Result<(), LoxError> {
        if let Some(scope) = self.scopes.last() {
            if let Some(false) = scope.get(&val.name.lexeme) {
                return Err(LoxError::SemanticError("Cannot read local variable in its own initializer.".to_string(), val.name.lineNo))
            }
        }
        self.resolveLocal(&mut val.name);
        Ok(())
    }

    fn visitVarStmt (&mut self, val: &mut Var) -> Result<(), LoxError> {
        self.declare(&mut val.name);
        if let Some(initl) = &mut val.initializer {
            self.resolve_exp(initl)?;
        }
        self.define(&mut val.name)?;
        self.resolveLocal(&mut val.name);
        Ok(())
    }

    fn visitAssignStmt (&mut self, val: &mut Assign) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.value)?;
        self.resolveLocal(&mut val.name);
        Ok(())
    }

    fn visitBlockStmt (&mut self, val: &mut Block) -> Result<(), LoxError> {
        self.beginScope();
        self.resolve(&mut val.statements)?;
        self.endScope();
        Ok(())
    }

    fn visitIfStmt (&mut self, val: &mut If) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.condition)?;
        self.resolve_st(&mut val.thenBranch)?;
        if let Some(elseBr) = &mut val.elseBranch {
            self.resolve_st(elseBr)?;
        }
        Ok(())
    }

    fn visitWhileStmt (&mut self, val: &mut While) -> Result<(), LoxError> {
        self.resolve_exp(&mut val.condition)?;
        self.resolve_st(&mut val.body)?;
        Ok(())
    }

    fn visitBreakStmt (&mut self, val: &mut Break) -> Result<(), LoxError> {
        Ok(())
    }

    fn visitContinueStmt (&mut self, val: &mut Continue) -> Result<(), LoxError> {
        Ok(())
    }

    fn visitFunctionStmt (&mut self, val: &mut Function) -> Result<(), LoxError> {
        self.declare(&mut val.name);
        self.define(&mut val.name)?;
        self.resolveFunction(val)?;
        Ok(())
    }

    fn visitReturnStmt (&mut self, val: &mut Return) -> Result<(), LoxError> {
        if let Some(val) = &mut val.value {
            self.resolve_exp(val)?;
        }
        Ok(())
    }

}