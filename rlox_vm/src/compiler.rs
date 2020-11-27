use crate::chunk::FuncSpec;
use crate::chunk::FunctionType;
use core::cell::RefCell;
use std::rc::Rc;
use rlox_core::frontend::definitions::token::Token;
use crate::chunk::Object;
use crate::VM;
use rlox_core::frontend::definitions::stmt::*;
use rlox_core::frontend::definitions::literal::Literal;
use rlox_core::frontend::definitions::expr::*;
use rlox_core::runtime::visitor::Visitor;
use rlox_core::runtime::visitor::VisAcceptor;
use rlox_core::frontend::resolver::Resolver;
use rlox_core::frontend::lexer::Lexer;
use rlox_core::frontend::parser::Parser;
use rlox_core::error::LoxError;
use crate::chunk::OpCode;
use std::fs::read_to_string;
use std::path::Path;
use rlox_core::frontend::definitions::token_type::TokenType;
use crate::chunk::Local;
pub fn run_file(path: &String) {
    let path = Path::new(path);
    let script = read_to_string(path).unwrap();
    if let Err(err) = run(&script) {
        println!("{}", err);
    }
}

fn run(script: &String) -> Result<(), LoxError> {
    let mut ast = Parser::new(Lexer::new().parse(script)?).parse()?;
    Resolver::new().resolve(&mut ast)?;
    // you have the ast here.
    let mut comp = Compiler::new();
    ast.accept(&mut comp)?;
    comp.curr_fn.chunks.push(OpCode::Exit(0));
    println!("{:?}", comp.curr_fn.chunks);
    let mut vm = VM::new(comp.constant_pool, comp.curr_fn);
    vm.run(false);
    Ok(())
}


struct Compiler{
    // pub chunk: Vec<OpCode>,
    pub constant_pool: Vec<Object>,
    pub curr_fn: FuncSpec 
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            // chunk: vec![],
            constant_pool: vec![],
            curr_fn: FuncSpec::new(0, None, FunctionType::SCRIPT),
            // scope_depth:0,
            // locals: vec![],
        }
    }
    pub fn add_const(&mut self, val: Object) -> usize {
        self.constant_pool.push(val);
        self.constant_pool.len()-1
    }
    fn is_true(&self, obj: &Object) -> bool
    where
        Self: Visitor<()>,
    {
        return match obj {
            Object::Bool(v) => *v,
            Object::Nil => false,
            _ => true,
        }
    }
    fn resolve_local(&mut self, token: &Token) -> i32 {
        for i in (0..self.curr_fn.locals.len()).rev() {
            if self.curr_fn.locals[i].name.lexeme == token.lexeme {
                return i as i32;
            }
        }
        return -1;
    }
    fn named_variable(&mut self, token: &Token) {
        let mut x = self.resolve_local(token);
        if x == -1 {
            x = self.add_const(Object::Str(token.lexeme.clone())) as i32;
            self.curr_fn.chunks.push(OpCode::GetGlobal(token.line_no, x as usize));
        } else {
            self.curr_fn.chunks.push(OpCode::GetLocal(token.line_no, x as usize));
        }
    }
    fn begin_scope(&mut self) {
        self.curr_fn.scope_depth += 1;
    }
    fn end_scope(&mut self) {
        self.curr_fn.scope_depth -= 1;
        while let Some(lc) = self.curr_fn.locals.last() {
            if lc.depth <= self.curr_fn.scope_depth {
                break;
            }
            self.curr_fn.chunks.push(OpCode::StackPop);
            self.curr_fn.locals.pop();
        }
    }
    fn declare_variable(&mut self, val: &Token) -> Result<(),LoxError>{
        if self.curr_fn.scope_depth == 0 {return Ok(())};
        let local = Local{name: val.clone(), depth: self.curr_fn.scope_depth};
        for lc in self.curr_fn.locals.iter().rev() {
            if lc.depth != -1 && lc.depth < self.curr_fn.scope_depth {
                break;
            }
            if local.name.lexeme == lc.name.lexeme {
                return Err(LoxError::RuntimeError(String::from(""), local.name.line_no, String::from("")))
            }
        }
        self.curr_fn.locals.push(local);
        Ok(())
    }
}

impl Visitor<()> for Compiler {
    fn visit_binary_expr(&mut self, val: &Binary) -> Result<(), LoxError> {
        val.left.accept(self)?;
        val.right.accept(self)?;
        match val.operator.token_type {
            TokenType::MINUS => self.curr_fn.chunks.push(OpCode::Subs(val.operator.line_no)),
            TokenType::PLUS => self.curr_fn.chunks.push(OpCode::Add(val.operator.line_no)),
            TokenType::STAR => self.curr_fn.chunks.push(OpCode::Multiply(val.operator.line_no)),
            TokenType::SLASH => self.curr_fn.chunks.push(OpCode::Divide(val.operator.line_no)),
            TokenType::EqualEqual => self.curr_fn.chunks.push(OpCode::EqualTo(val.operator.line_no)),
            TokenType::BangEqual => self.curr_fn.chunks.push(OpCode::NotEqualTo(val.operator.line_no)),
            TokenType::GREATER => self.curr_fn.chunks.push(OpCode::GreaterThan(val.operator.line_no)),
            TokenType::GreaterEqual => self.curr_fn.chunks.push(OpCode::GreaterThanEq(val.operator.line_no)),
            TokenType::LESS => self.curr_fn.chunks.push(OpCode::LesserThan(val.operator.line_no)),
            TokenType::LessEqual => self.curr_fn.chunks.push(OpCode::LesserThanEq(val.operator.line_no)),
            _ => return Err(LoxError::SemanticError(String::from(""),val.operator.line_no,String::from("")))
        }
        Ok(())
    }

    fn visit_call_expr(&mut self, val: &Call) -> Result<(), LoxError> {
        val.callee.accept(self)?;
        for arg in &val.arguments {
            arg.accept(self)?;
        }
        self.curr_fn.chunks.push(OpCode::FnCall(0, val.arguments.len()));
        Ok(())
    }

    fn visit_grouping_expr(&mut self, val: &Grouping) -> Result<(), LoxError> {
        val.expression.accept(self)
    }

    fn visit_unary_expr(&mut self, val: &Unary) -> Result<(), LoxError> {
        if val.operator.token_type != TokenType::MINUS {
            return Err(LoxError::RuntimeError(String::from(""), val.operator.line_no, String::from("")))
        }
        val.right.accept(self)?;
        self.curr_fn.chunks.push(OpCode::Negate(val.operator.line_no));
        Ok(())
    }

    fn visit_literal_expr(&mut self, val: &Literal) -> Result<(), LoxError> {
        let x = self.add_const(val.clone().into());
        // println!("{:?} -> {}", val, x);
        self.curr_fn.chunks.push(OpCode::Constant(0, x));
        Ok(())
    }

    fn visit_logical_expr(&mut self, val: &Logical) -> Result<(), LoxError> {
        val.left.accept(self)?;
        val.right.accept(self)?;
        match val.operator.token_type {
            TokenType::AND => self.curr_fn.chunks.push(OpCode::BoolAnd(val.operator.line_no)),
            TokenType::OR => self.curr_fn.chunks.push(OpCode::BoolOr(val.operator.line_no)),
            _ => return Err(LoxError::RuntimeError(String::from(""), val.operator.line_no, String::from("")))
        };
        Ok(())
    }

    fn visit_get_expr(&mut self, val: &Get) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_set_expr(&mut self, val: &Set) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_lambda_expr(&mut self, val: &Lambda) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_this_expr(&mut self, val: &This) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_super_expr(&mut self, val: &Super) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_expression_stmt(&mut self, val: &Expression) -> Result<(), LoxError> {
        // println!("{:?}", val);
        val.expr.accept(self)?;
        self.curr_fn.chunks.push(OpCode::StackPop);
        Ok(())
    }

    fn visit_print_stmt(&mut self, val: &Print) -> Result<(), LoxError> {
        val.expr.accept(self)?;
        self.curr_fn.chunks.push(OpCode::Print(0));
        Ok(())
    }

    fn visit_variable_stmt(&mut self, val: &Variable) -> Result<(), LoxError> {
        self.named_variable(&val.name);
        Ok(())
    }

    fn visit_var_stmt(&mut self, val: &Var) -> Result<(), LoxError> {
        if let Some(init) = &val.initializer {
            init.accept(self)?;
        } else {
            self.curr_fn.chunks.push(OpCode::NilVal);
        }
        // println!("{} {}", val.name.lexeme, self.curr_fn.scope_depth);
        // special handling of locals
        self.declare_variable(&val.name)?;
        if self.curr_fn.scope_depth > 0 {return Ok(())}
        
        let x = self.add_const(Object::Str(val.name.lexeme.clone()));
        self.curr_fn.chunks.push(OpCode::DefineGlobal(val.name.line_no, x));

        Ok(())
    }

    fn visit_assign_stmt(&mut self, val: &Assign) -> Result<(), LoxError> {
        val.value.accept(self)?;

        let mut x = self.resolve_local(&val.name);
        if x == -1 {
            // TODO: think of better approach
            x = self.add_const(Object::Str(val.name.lexeme.clone())) as i32;
            self.curr_fn.chunks.push(OpCode::SetGlobal(val.name.line_no, x as usize));
        } else {
            self.curr_fn.chunks.push(OpCode::SetLocal(val.name.line_no, x as usize));
        }

        Ok(())
    }

    fn visit_block_stmt(&mut self, val: &Block) -> Result<(), LoxError> {
        self.begin_scope();
        val.statements.accept(self)?;
        self.end_scope();
        Ok(())
    }
    fn visit_if_stmt(&mut self, val: &If) -> Result<(), LoxError> {
        val.condition.accept(self)?;
        // change the 9999 to some sentinel value, in compile mode verify the jump does not point to any sentinel value.
        self.curr_fn.chunks.push(OpCode::JumpIfFalse(0, 9999));
        let then_jump = self.curr_fn.chunks.len()-1;
        self.curr_fn.chunks.push(OpCode::StackPop);

        val.then_branch.accept(self)?;

        self.curr_fn.chunks.push(OpCode::Jump(0, 9999));
        let else_jump = self.curr_fn.chunks.len()-1;
        
        self.curr_fn.chunks[then_jump] = OpCode::JumpIfFalse(0, self.curr_fn.chunks.len()-1);
        self.curr_fn.chunks.push(OpCode::StackPop);

        if let Some(else_branch) = &val.else_branch {
            else_branch.accept(self)?;
        }
        self.curr_fn.chunks[else_jump] = OpCode::Jump(0, self.curr_fn.chunks.len()-1);

        Ok(())
    }

    fn visit_while_stmt(&mut self, val: &While) -> Result<(), LoxError> {
        let loop_start = self.curr_fn.chunks.len();
        self.curr_fn.chunks.push(OpCode::NoOp);
        val.condition.accept(self)?;
        
        self.curr_fn.chunks.push(OpCode::JumpIfFalse(0, 9999));
        let then_jump = self.curr_fn.chunks.len()-1;
        self.curr_fn.chunks.push(OpCode::StackPop);

        val.body.accept(self)?;

        self.curr_fn.chunks.push(OpCode::Jump(0, loop_start));
        self.curr_fn.chunks[then_jump] = OpCode::JumpIfFalse(0, self.curr_fn.chunks.len());
        self.curr_fn.chunks.push(OpCode::StackPop);
        Ok(())
    }

    fn visit_break_stmt(&mut self, val: &Break) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_continue_stmt(&mut self, val: &Continue) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_function_stmt(&mut self, val: &Function) -> Result<(), LoxError> {
        let mut new_func = FuncSpec::new(0, Some(val.name.lexeme.clone()), FunctionType::FUNCTION);
        new_func = std::mem::replace(&mut self.curr_fn, new_func);
        self.begin_scope();
        
        self.curr_fn.arity = val.params.len() as u32;
        for param in &val.params {
            self.declare_variable(&param)?;
        }

        val.body.accept(self)?;
        
        self.curr_fn.chunks.push(OpCode::NilVal);
        self.curr_fn.chunks.push(OpCode::Return(0));

        self.end_scope();
        // original new func
        new_func = std::mem::replace(&mut self.curr_fn, new_func);
        println!("fn: {:?}", new_func);
        // self.declare_variable(&val.name)?;
        // if self.curr_fn.scope_depth > 0 {return Ok(())}
        
        let x = self.add_const(Object::Str(val.name.lexeme.clone()));
        let y = self.add_const(Object::Function(new_func));
        self.curr_fn.chunks.push(OpCode::Constant(0, y));
        self.curr_fn.chunks.push(OpCode::DefineGlobal(val.name.line_no, x));

        Ok(())
    }

    fn visit_return_stmt(&mut self, val: &Return) -> Result<(), LoxError> {
        if let Some(vl) = &val.value {
            vl.accept(self)?;
            self.curr_fn.chunks.push(OpCode::Return(0));
        } else {
            self.curr_fn.chunks.push(OpCode::NilVal);
            self.curr_fn.chunks.push(OpCode::Return(0));
        }
        Ok(())
    }

    fn visit_class_stmt(&mut self, val: &Class) -> Result<(), LoxError> {
        todo!();
    }
}