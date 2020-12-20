use rlox_core::frontend::definitions::expr::*;
use rlox_core::frontend::definitions::stmt::*;
use rlox_core::frontend::lexer::*;

use rlox_core::runtime::visitor::{VisitorMut, VisitorMutAcceptor};

use std::collections::HashMap;
use rlox_core::frontend::definitions::function_type::FunctionType;
use rlox_core::frontend::definitions::class_type::ClassType;
use rlox_core::error::LoxError;
use rlox_core::frontend::definitions::literal::Literal;
use rlox_core::frontend::definitions::token::Token;

// handle break/continue at resolve
// static fields maybe

pub struct Resolver {
    // bool corresponds to if the value has been initialized
    pub fn_scopes: Vec<Vec<HashMap<String, bool>>>,
    curr_class: ClassType,
    curr_function: FunctionType,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            fn_scopes: vec![vec![HashMap::new()]],
            curr_class: ClassType::NONE,
            curr_function: FunctionType::NONE,
        }
    }

    pub fn resolve<T: VisitorMutAcceptor<()> + Sized>(
        &mut self,
        expr: &mut T,
    ) -> Result<(), LoxError> {
        expr.accept(self)
    }

    fn resolve_local(&mut self, name: &mut Token) -> Result<(), LoxError> {
        for (i, scopes) in self.fn_scopes.iter().enumerate().rev() {
            for (_, scope) in scopes.iter().enumerate().rev() {
                if scope.contains_key(&name.lexeme) {
                    name.scope = Some(self.fn_scopes.len() - 1 - i);
                    // println!("resolver {:?} {:?}", name, self.fn_scopes);
                    return Ok(());
                }
            }
        }
        return Err(LoxError::SemanticError(name.lexeme.clone(),name.line_no, String::from("Undefined")));
    }

    // not for block scopes only lexical
    fn begin_scope(&mut self) {
        self.fn_scopes.last_mut().unwrap().push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.fn_scopes.last_mut().unwrap().pop();
    }

    fn begin_fn_scope(&mut self) {
        self.fn_scopes.push(vec![HashMap::new()]);
        self.begin_scope();
    }

    fn end_fn_scope(&mut self) {
        self.end_scope();
        self.fn_scopes.pop();
    }

    fn declare(&mut self, token: &Token) -> Result<(), LoxError> {
        if let Some(scopes) = self.fn_scopes.last_mut() {
            if let Some(scope) = scopes.last_mut() {
                if scope.insert(token.lexeme.clone(), false).is_some() {
                    return Err(LoxError::SemanticError(token.lexeme.clone(),token.line_no, String::from("Already exists")));
                }
            }
        }
        //TODO: else err, actually perform this check on every `if let`
        Ok(())
    }

    fn define(&mut self, token: &Token) -> Result<(), LoxError> {
        if let Some(scopes) = self.fn_scopes.last_mut() {
            if let Some(scope) = scopes.last_mut() {
                scope
                    .insert(token.lexeme.clone(), true)
                    .ok_or(LoxError::SemanticError(
                        token.lexeme.clone(),
                        token.line_no,
                        "Declaration without definition".to_string(),
                    ))?;
            }
        }
        Ok(())
    }

    fn resolve_function(&mut self, func: &mut Function, dec: FunctionType) -> Result<(), LoxError> {
        self.begin_fn_scope();
        let currfn = self.curr_function;
        self.curr_function = dec;

        for param in &func.params {
            self.declare(&param)?;
            self.define(&param)?;
        }
        self.resolve(&mut func.body)?;
        self.end_fn_scope();
        self.curr_function = currfn;
        Ok(())
    }

    fn resolve_lambda(&mut self, func: &mut Lambda) -> Result<(), LoxError> {
        self.begin_fn_scope();
        let currfn = self.curr_function;
        self.curr_function = FunctionType::LAMBDA;

        for param in &func.params {
            self.declare(&param)?;
            self.define(&param)?;
        }
        self.resolve(&mut func.body)?;
        self.end_fn_scope();
        self.curr_function = currfn;
        Ok(())
    }
}

impl VisitorMut<()> for Resolver {
    fn visit_binary_expr(&mut self, val: &mut Binary) -> Result<(), LoxError> {
        self.resolve(&mut val.left)?;
        self.resolve(&mut val.right)?;
        Ok(())
    }

    fn visit_call_expr(&mut self, val: &mut Call) -> Result<(), LoxError> {
        // This is so that recursion and stuff works in peace
        // self.resolve(&mut val.callee)?;
        for arg in &mut val.arguments {
            self.resolve(arg)?;
        }
        Ok(())
    }

    fn visit_grouping_expr(&mut self, val: &mut Grouping) -> Result<(), LoxError> {
        self.resolve(&mut val.expression)?;
        Ok(())
    }

    fn visit_unary_expr(&mut self, val: &mut Unary) -> Result<(), LoxError> {
        self.resolve(&mut val.right)?;
        Ok(())
    }

    fn visit_literal_expr(&mut self, _val: &mut Literal) -> Result<(), LoxError> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, val: &mut Logical) -> Result<(), LoxError> {
        self.resolve(&mut val.left)?;
        self.resolve(&mut val.right)?;
        Ok(())
    }

    fn visit_get_expr(&mut self, val: &mut Get) -> Result<(), LoxError> {
        self.resolve(&mut val.object)?;
        Ok(())
    }

    fn visit_set_expr(&mut self, val: &mut Set) -> Result<(), LoxError> {
        self.resolve(&mut val.value)?;
        self.resolve(&mut val.object)?;
        Ok(())
    }

    fn visit_lambda_expr(&mut self, val: &mut Lambda) -> Result<(), LoxError> {
        self.resolve_lambda(val)?;
        Ok(())
    }

    fn visit_this_expr(&mut self, val: &mut This) -> Result<(), LoxError> {
        if self.curr_class == ClassType::NONE {
            return Err(LoxError::SemanticError(
                val.keyword.lexeme.clone(),
                val.keyword.line_no,
                "Cannot use this outside class.".to_string(),
            ));
        }
        self.resolve_local(&mut val.keyword)?;
        Ok(())
    }

    fn visit_super_expr(&mut self, val: &mut Super) -> Result<(), LoxError> {
        if self.curr_class != ClassType::SUBCLASS {
            return Err(LoxError::SemanticError(
                val.keyword.lexeme.clone(),
                val.keyword.line_no,
                "No super class as such.".to_string(),
            ));
        }
        self.resolve_local(&mut val.keyword)?;
        Ok(())
    }

    fn visit_expression_stmt(&mut self, val: &mut Expression) -> Result<(), LoxError> {
        self.resolve(&mut val.expr)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, val: &mut Print) -> Result<(), LoxError> {
        self.resolve(&mut val.expr)?;
        Ok(())
    }

    fn visit_variable_stmt(&mut self, val: &mut Variable) -> Result<(), LoxError> {
        if let Some(scopes) = self.fn_scopes.last_mut() {
            if let Some(scope) = scopes.last_mut() {
                if let Some(false) = scope.get(&val.name.lexeme) {
                    return Err(LoxError::SemanticError(
                        val.name.lexeme.clone(),
                        val.name.line_no,
                        "Cannot read local variable in its own initializer.".to_string(),
                    ));
                }
            }
        }
        self.resolve_local(&mut val.name)?;
        Ok(())
    }

    fn visit_var_stmt(&mut self, val: &mut Var) -> Result<(), LoxError> {
        self.declare(&mut val.name)?;
        if let Some(initl) = &mut val.initializer {
            self.resolve(initl)?;
        }
        self.define(&mut val.name)?;
        self.resolve_local(&mut val.name)?;
        Ok(())
    }

    fn visit_assign_stmt(&mut self, val: &mut Assign) -> Result<(), LoxError> {
        self.resolve(&mut val.value)?;
        self.resolve_local(&mut val.name)?;
        Ok(())
    }

    fn visit_block_stmt(&mut self, val: &mut Block) -> Result<(), LoxError> {
        self.begin_scope();
        self.resolve(&mut val.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_if_stmt(&mut self, val: &mut If) -> Result<(), LoxError> {
        self.resolve(&mut val.condition)?;
        self.resolve(&mut val.then_branch)?;
        if let Some(else_br) = &mut val.else_branch {
            self.resolve(else_br)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, val: &mut While) -> Result<(), LoxError> {
        self.resolve(&mut val.condition)?;
        match self.resolve(&mut val.body) {
            Err(LoxError::Break(_)) | Err(LoxError::Continue(_)) | Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn visit_break_stmt(&mut self, val: &mut Break) -> Result<(), LoxError> {
        Err(LoxError::Break(val.keyword.line_no))
    }

    fn visit_continue_stmt(&mut self, val: &mut Continue) -> Result<(), LoxError> {
        Err(LoxError::Continue(val.keyword.line_no))
    }

    fn visit_function_stmt(&mut self, val: &mut Function) -> Result<(), LoxError> {
        self.declare(&mut val.name)?;
        self.define(&mut val.name)?;
        self.resolve_function(val, FunctionType::FUNCTION)?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, val: &mut Return) -> Result<(), LoxError> {
        if let Some(value) = &mut val.value {
            if self.curr_function == FunctionType::NONE {
                return Err(LoxError::SemanticError(
                    val.keyword.lexeme.clone(),
                    val.keyword.line_no,
                    "Return only from function".to_string(),
                ));
            }
            if self.curr_function == FunctionType::INITIALIZER {
                return Err(LoxError::SemanticError(
                    val.keyword.lexeme.clone(),
                    val.keyword.line_no,
                    "Cannot return from initializer".to_string(),
                ));
            }
            self.resolve(value)?;
        }
        Ok(())
    }

    fn visit_class_stmt(&mut self, val: &mut Class) -> Result<(), LoxError> {
        let curr_class = self.curr_class;
        self.curr_class = ClassType::CLASS;
        self.declare(&val.name)?;
        self.define(&val.name)?;
        self.resolve_local(&mut val.name)?;
        if let Some(sp_class) = &mut val.superclass {
            if sp_class.name.lexeme == val.name.lexeme {
                return Err(LoxError::SemanticError(
                    val.name.lexeme.clone(),
                    val.name.line_no,
                    "Class can't inherit itself".to_string(),
                ));
            }
            self.curr_class = ClassType::SUBCLASS;
            // self.resolve(&mut Expr::Variable(Box::new(_sp_class.clone())))?;
            self.visit_variable_stmt(sp_class)?;
            self.begin_scope();
            self.fn_scopes
                .last_mut()
                .unwrap()
                .last_mut()
                .unwrap()
                .insert("super".to_string(), true);
        }

        self.begin_scope();
        self.fn_scopes
            .last_mut()
            .unwrap()
            .last_mut()
            .unwrap()
            .insert("this".to_string(), true);
        for method in &mut val.methods {
            let dec = if method.name.lexeme == "init".to_string() {
                FunctionType::INITIALIZER
            } else {
                FunctionType::METHOD
            };
            self.resolve_function(method, dec)?;
        }

        self.end_scope();
        if let Some(_) = &val.superclass {
            self.end_scope();
        }
        self.curr_class = curr_class;
        Ok(())
    }

    fn visit_stack_trace_stmt(&mut self) -> Result<(), LoxError> {
        // todo!()
        Ok(())
    }
}
