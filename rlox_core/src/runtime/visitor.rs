use crate::error::LoxError;
use crate::frontend::definitions::expr::{
    Assign, Binary, Call, Expr, Get, Grouping, Lambda, Logical, Set, Super, This, Unary, Variable,
};
use crate::frontend::definitions::literal::Literal;
use crate::frontend::definitions::stmt::{
    Block, Break, Class, Continue, Expression, Function, If, Print, Return, Stmt, Var, While,
};

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
    fn visit_stack_trace_stmt(&mut self) -> Result<R, LoxError>;
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
            Stmt::StackTrace => vis.visit_stack_trace_stmt(),
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
    fn visit_stack_trace_stmt(&mut self) -> Result<R, LoxError>;
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
            Stmt::StackTrace => vis.visit_stack_trace_stmt(),
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
