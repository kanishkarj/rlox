use crate::{chunk::FuncSpec, gc::{root::CustomClone, heap::Heap, root::Trace}};
use crate::chunk::FunctionType;
use core::cell::RefCell;
use rlox_core::frontend::definitions::token::Token;
use crate::chunk::Object;
use crate::chunk::VM;
use rlox_core::frontend::definitions::stmt::*;
use rlox_core::frontend::definitions::literal::Literal;
use rlox_core::frontend::definitions::expr::*;
use rlox_core::runtime::visitor::Visitor;
use rlox_core::runtime::visitor::VisAcceptor;
use crate::resolver::Resolver;
use rlox_core::frontend::lexer::Lexer;
use rlox_core::frontend::parser::Parser;
use rlox_core::error::LoxError;
use crate::chunk::OpCode;
use std::fs::read_to_string;
use std::path::Path;
use rlox_core::frontend::definitions::token_type::TokenType;
use crate::chunk::Local;
use crate::system_calls::SystemCalls;

pub fn run_file<T: SystemCalls, S: AsRef<str>>(path: S, sys_interface: T)  -> Result<(), LoxError> {
    let path = Path::new(path.as_ref());
    let script = read_to_string(path).unwrap();
    run(&script, sys_interface)?;
    Ok(())
    // if let Err(err) = run(&script, sys_interface) {
    //     println!("{}", err);
    // }
}

fn run<T: SystemCalls, S: AsRef<str>>(script: S, sys_interface: T) -> Result<(), LoxError> {
    let x = script.as_ref().to_string();
    let mut ast = Parser::new(Lexer::new().parse(&x)?).parse()?;
    Resolver::new().resolve(&mut ast)?;
    // you have the ast here.
    let mut gc = Heap::new();
    let mut comp = Compiler::new(&gc);
    ast.accept(&mut comp)?;
    comp.curr_fn_mut().chunks.push(OpCode::Exit(0));
    println!("{:?}", comp.curr_fn().chunks);
    println!("hello");
    let curr_fn = comp.curr_fn().clone(&gc);
    let cons_pl = comp.constant_pool;
    let mut vm = VM::new(sys_interface, cons_pl, curr_fn, &gc);
    vm.run(false, &gc)?;
    Ok(())
}


struct Compiler<'a>{
    // pub chunk: Vec<OpCode>,
    pub constant_pool: Vec<Object>,
    pub scoped_fns: Vec<FuncSpec>,
    pub gc: &'a Heap,
}

impl<'a> Compiler<'a> {
    pub fn new(gc: &'a Heap) -> Self {
        Compiler {
            constant_pool: vec![],
            scoped_fns: vec![FuncSpec::new(0, None, FunctionType::SCRIPT)],
            gc
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
        for i in (0..self.curr_fn().locals.len()).rev() {
            if self.curr_fn().locals[i].name.lexeme == token.lexeme {
                return i as i32;
            }
        }
        return -1;
    }
    fn named_variable(&mut self, token: &Token) {
        let mut x = self.resolve_local(token);
        if x == -1 {
            x = self.resolve_upvalue(token);
            // println!("{:?} {} {:?} {}", self.curr_fn().name, token.lexeme ,token.scope, x);
            if x != -1 {
                self.curr_fn_mut().chunks.push(OpCode::GetUpvalue(token.line_no, x as usize));
            } else {
                x = self.add_const(Object::Str(token.lexeme.clone())) as i32;
                self.curr_fn_mut().chunks.push(OpCode::GetGlobal(token.line_no, x as usize));
            }
        } else {
            self.curr_fn_mut().chunks.push(OpCode::GetLocal(token.line_no, x as usize));
        }
    }
    fn begin_scope(&mut self) {
        self.curr_fn_mut().scope_depth += 1;
    }
    fn end_scope(&mut self) {
        self.curr_fn_mut().scope_depth -= 1;
        while let Some(lc) = self.curr_fn().locals.last() {
            if lc.depth <= self.curr_fn().scope_depth {
                break;
            }
            if self.curr_fn().locals.last().unwrap().is_closed {
                self.curr_fn_mut().chunks.push(OpCode::CloseUpvalue);
            } else {
                self.curr_fn_mut().chunks.push(OpCode::StackPop);
            }
            self.curr_fn_mut().locals.pop();
        }
    }
    fn declare_variable(&mut self, val: &Token) -> Result<(),LoxError>{
        if self.curr_fn().scope_depth == 0 {return Ok(())};
        let local = Local{name: val.clone(), depth: self.curr_fn().scope_depth, is_closed: false};
        for lc in self.curr_fn().locals.iter().rev() {
            if lc.depth != -1 && lc.depth < self.curr_fn().scope_depth {
                break;
            }
            if local.name.lexeme == lc.name.lexeme {
                return Err(LoxError::RuntimeError(String::from(""), local.name.line_no, String::from("")))
            }
        }
        self.curr_fn_mut().locals.push(local);
        Ok(())
    }
    
    fn curr_fn(&self) -> &FuncSpec {
        self.scoped_fns.last().unwrap()
    }

    fn curr_fn_mut(&mut self) -> &mut FuncSpec {
        self.scoped_fns.last_mut().unwrap()
    }

    fn resolve_upvalue(&mut self, token: &Token) -> i32 {
        
        let n = self.scoped_fns.len();
        if n < 2 {
            return -1;
        }

        
        if let Some(mut scope) = token.scope {
            scope = n - scope;
            let mut upval = -1;
            for j in (0..self.scoped_fns[scope-1].locals.len()).rev() {
                if self.scoped_fns[scope-1].locals[j].name.lexeme == token.lexeme {
                    self.scoped_fns[scope-1].locals[j].is_closed = true;
                    upval = self.scoped_fns[scope].add_upvalue(j, true) as i32;
                    break;
                }
            }
            if upval != -1 {
                for i in scope+1..n {
                    upval = self.scoped_fns[i].add_upvalue(upval as usize, false) as i32;
                }
            }
            return upval;
        }

        return -1;
    }
}

impl<'a> Visitor<()> for Compiler<'a> {
    fn visit_binary_expr(&mut self, val: &Binary) -> Result<(), LoxError> {
        val.left.accept(self)?;
        val.right.accept(self)?;
        match val.operator.token_type {
            TokenType::MINUS => self.curr_fn_mut().chunks.push(OpCode::Subs(val.operator.line_no)),
            TokenType::PLUS => self.curr_fn_mut().chunks.push(OpCode::Add(val.operator.line_no)),
            TokenType::STAR => self.curr_fn_mut().chunks.push(OpCode::Multiply(val.operator.line_no)),
            TokenType::SLASH => self.curr_fn_mut().chunks.push(OpCode::Divide(val.operator.line_no)),
            TokenType::EqualEqual => self.curr_fn_mut().chunks.push(OpCode::EqualTo(val.operator.line_no)),
            TokenType::BangEqual => self.curr_fn_mut().chunks.push(OpCode::NotEqualTo(val.operator.line_no)),
            TokenType::GREATER => self.curr_fn_mut().chunks.push(OpCode::GreaterThan(val.operator.line_no)),
            TokenType::GreaterEqual => self.curr_fn_mut().chunks.push(OpCode::GreaterThanEq(val.operator.line_no)),
            TokenType::LESS => self.curr_fn_mut().chunks.push(OpCode::LesserThan(val.operator.line_no)),
            TokenType::LessEqual => self.curr_fn_mut().chunks.push(OpCode::LesserThanEq(val.operator.line_no)),
            _ => return Err(LoxError::SemanticError(String::from(""),val.operator.line_no,String::from("")))
        }
        Ok(())
    }

    fn visit_call_expr(&mut self, val: &Call) -> Result<(), LoxError> {
        val.callee.accept(self)?;
        for arg in &val.arguments {
            arg.accept(self)?;
        }
        self.curr_fn_mut().chunks.push(OpCode::Call(val.paren.line_no, val.arguments.len()));
        Ok(())
    }

    fn visit_grouping_expr(&mut self, val: &Grouping) -> Result<(), LoxError> {
        val.expression.accept(self)
    }

    fn visit_unary_expr(&mut self, val: &Unary) -> Result<(), LoxError> {
        if val.operator.token_type == TokenType::MINUS {
            val.right.accept(self)?;
            self.curr_fn_mut().chunks.push(OpCode::Negate(val.operator.line_no));
            Ok(())
        } else if val.operator.token_type == TokenType::BANG {
            val.right.accept(self)?;
            self.curr_fn_mut().chunks.push(OpCode::Not(val.operator.line_no));
            Ok(())
        } else {
            Err(LoxError::RuntimeError(String::from(""), val.operator.line_no, String::from("")))
        }
    }

    fn visit_literal_expr(&mut self, val: &Literal) -> Result<(), LoxError> {
        let x = self.add_const(val.clone().into());
        self.curr_fn_mut().chunks.push(OpCode::Constant(x));
        Ok(())
    }

    fn visit_logical_expr(&mut self, val: &Logical) -> Result<(), LoxError> {
        val.left.accept(self)?;
        val.right.accept(self)?;
        match val.operator.token_type {
            TokenType::AND => self.curr_fn_mut().chunks.push(OpCode::BoolAnd(val.operator.line_no)),
            TokenType::OR => self.curr_fn_mut().chunks.push(OpCode::BoolOr(val.operator.line_no)),
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
        val.expr.accept(self)?;
        self.curr_fn_mut().chunks.push(OpCode::StackPop);
        Ok(())
    }

    fn visit_print_stmt(&mut self, val: &Print) -> Result<(), LoxError> {
        val.expr.accept(self)?;
        self.curr_fn_mut().chunks.push(OpCode::Print(val.token.line_no));
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
            self.curr_fn_mut().chunks.push(OpCode::NilVal);
        }
        // println!("{} {}", val.name.lexeme, self.curr_fn().scope_depth);
        // special handling of locals
        self.declare_variable(&val.name)?;
        if self.curr_fn().scope_depth > 0 {return Ok(())}
        
        let x = self.add_const(Object::Str(val.name.lexeme.clone()));
        self.curr_fn_mut().chunks.push(OpCode::DefineGlobal(val.name.line_no, x));

        Ok(())
    }

    fn visit_assign_stmt(&mut self, val: &Assign) -> Result<(), LoxError> {
        val.value.accept(self)?;

        let mut x = self.resolve_local(&val.name);
        if x == -1 {

            x = self.resolve_upvalue(&val.name);
            if x != -1 {
                self.curr_fn_mut().chunks.push(OpCode::SetUpvalue(val.name.line_no, x as usize));
            } else {
                // TODO: think of better approach
                x = self.add_const(Object::Str(val.name.lexeme.clone())) as i32;
                self.curr_fn_mut().chunks.push(OpCode::SetGlobal(val.name.line_no, x as usize));
                // x = self.add_const(Object::Str(val.name.lexeme.clone())) as i32;
                // self.curr_fn_mut().chunks.push(OpCode::GetGlobal(val.name.line_no, x as usize));
            }

        } else {
            self.curr_fn_mut().chunks.push(OpCode::SetLocal(val.name.line_no, x as usize));
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
        self.curr_fn_mut().chunks.push(OpCode::JumpIfFalse(val.token.line_no, 9999));
        let then_jump = self.curr_fn().chunks.len()-1;
        self.curr_fn_mut().chunks.push(OpCode::StackPop);

        val.then_branch.accept(self)?;

        self.curr_fn_mut().chunks.push(OpCode::Jump(val.token.line_no, 9999));
        let else_jump = self.curr_fn().chunks.len()-1;
        
        self.curr_fn_mut().chunks[then_jump] = OpCode::JumpIfFalse(val.token.line_no, self.curr_fn().chunks.len());
        self.curr_fn_mut().chunks.push(OpCode::StackPop);

        if let Some(else_branch) = &val.else_branch {
            else_branch.accept(self)?;
        }
        self.curr_fn_mut().chunks[else_jump] = OpCode::Jump(val.token.line_no, self.curr_fn().chunks.len());

        Ok(())
    }

    fn visit_while_stmt(&mut self, val: &While) -> Result<(), LoxError> {
        let loop_start = self.curr_fn().chunks.len()+1;
        self.curr_fn_mut().chunks.push(OpCode::NoOp);
        val.condition.accept(self)?;
        
        self.curr_fn_mut().chunks.push(OpCode::JumpIfFalse(val.token.line_no, 9999));
        let then_jump = self.curr_fn().chunks.len()-1;
        self.curr_fn_mut().chunks.push(OpCode::StackPop);

        val.body.accept(self)?;

        self.curr_fn_mut().chunks.push(OpCode::Jump(val.token.line_no, loop_start));
        self.curr_fn_mut().chunks[then_jump] = OpCode::JumpIfFalse(val.token.line_no, self.curr_fn().chunks.len());
        self.curr_fn_mut().chunks.push(OpCode::StackPop);
        Ok(())
    }

    fn visit_break_stmt(&mut self, val: &Break) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_continue_stmt(&mut self, val: &Continue) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_function_stmt(&mut self, val: &Function) -> Result<(), LoxError> {
        self.scoped_fns.push(FuncSpec::new(0, Some(val.name.lexeme.clone()), FunctionType::FUNCTION));
        self.begin_scope();
        
        self.curr_fn_mut().arity = val.params.len() as u32;
        for param in &val.params {
            self.declare_variable(&param)?;
        }

        val.body.accept(self)?;
        
        self.curr_fn_mut().chunks.push(OpCode::NilVal);
        self.curr_fn_mut().chunks.push(OpCode::Return(val.name.line_no));

        self.end_scope();

        // original new func
        let new_func = self.scoped_fns.pop().unwrap();
        // println!("name:{:?} ||| upvals:{:?}", new_func.name, new_func.upvalues);
        // println!("fn: {:?}", new_func);
        // self.declare_variable(&val.name)?;
        // if self.curr_fn().scope_depth > 0 {return Ok(())}
        
        // let x = self.add_const(Object::Str(val.name.lexeme.clone()));
        let func_root = self.gc.get_unique_root(new_func);
        let y = self.add_const(Object::Closure(func_root));
        // self.curr_fn_mut().chunks.push(OpCode::Constant(y));
        self.curr_fn_mut().chunks.push(OpCode::Closure(val.name.line_no, y));
        //self.curr_fn_mut().chunks.push(OpCode::DefineGlobal(val.name.line_no, x));
        self.declare_variable(&val.name)?;
        println!("{}", self.curr_fn().scope_depth);
        if self.curr_fn().scope_depth > 0 {return Ok(())}
        
        let x = self.add_const(Object::Str(val.name.lexeme.clone()));
        self.curr_fn_mut().chunks.push(OpCode::DefineGlobal(val.name.line_no, x));
        self.curr_fn_mut().chunks.push(OpCode::StackPop);

        Ok(())
    }

    fn visit_return_stmt(&mut self, val: &Return) -> Result<(), LoxError> {
        if let Some(vl) = &val.value {
            vl.accept(self)?;
            self.curr_fn_mut().chunks.push(OpCode::Return(val.keyword.line_no));
        } else {
            self.curr_fn_mut().chunks.push(OpCode::NilVal);
            self.curr_fn_mut().chunks.push(OpCode::Return(val.keyword.line_no));
        }
        Ok(())
    }

    fn visit_class_stmt(&mut self, val: &Class) -> Result<(), LoxError> {
        todo!();
    }

    fn visit_stack_trace_stmt(&mut self) -> Result<(), LoxError> {
        self.curr_fn_mut().chunks.push(OpCode::PrintStackTrace);
        Ok(())
    }
}