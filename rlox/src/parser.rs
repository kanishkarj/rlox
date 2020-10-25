use crate::grammar::expr::*;
use crate::grammar::stmt::*;
use crate::runner::Runner;
use crate::scanner::*;
use crate::token_type::TokenType;
use crate::error::LoxError;
use crate::literal::Literal;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

// TODO: Look into how enum matching is happening

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, curr: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            let val = self.declaration();
            match val {
                Ok(val) => statements.push(val),
                Err(err) => {
                    self.synchronize();
                    return Err(err);
                }
            }
        }
        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.validate(TokenType::CLASS) {
            return self.class_declaration();
        }
        if self.validate(TokenType::FUN) {
            return self.function("function");
        }
        if self.validate(TokenType::VAR) {
            return self.val_declaration();
        }
        return self.statement();
    }

    fn class_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self
            .consume(TokenType::IDENTIFIER, format!("Expect Class name"))?
            .clone();
        let mut super_class = None;

        if self.validate(TokenType::LESS) {
            self.consume(TokenType::IDENTIFIER, format!("Expect SuperClass name"))?;
            super_class = Some(Variable::new(self.previous().clone()));
        }
        self.consume(
            TokenType::LeftBrace,
            format!("Expect {{ before class body"),
        )?;

        let mut methods = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Stmt::Function(method) = self.function("method")? {
                methods.push(*method);
            }
        }

        self.consume(
            TokenType::RightBrace,
            "Expect '}' after class body".to_string(),
        )?;
        return Ok(Stmt::Class(Box::new(Class::new(
            name.clone(),
            methods,
            super_class,
        ))));
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, LoxError> {
        let name = self
            .consume(TokenType::IDENTIFIER, format!("Expect {} name", kind))?
            .clone();
        self.consume(TokenType::LeftParen, format!("Expect after {} name", kind))?;
        let mut params = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() > 255 {
                    return Err(Self::error(
                        self.peek().clone(),
                        "m ax no. of args 255".to_string(),
                    ));
                }
                params.push(
                    self.consume(TokenType::IDENTIFIER, "Expect Param Name".to_string())?
                        .clone(),
                );
                if !self.validate(TokenType::COMMA) {
                    break;
                };
            }
        }
        self.consume(
            TokenType::RightParen,
            "Expect ')' after params".to_string(),
        )?;
        self.consume(TokenType::LeftBrace, "Expect '{' before body".to_string())?;

        let body = self.block()?;
        return Ok(Stmt::Function(Box::new(Function::new(
            name.clone(),
            params,
            body,
        ))));
    }

    fn val_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::IDENTIFIER, "Expect Variable Name.".to_string())?;
        let name = name.clone();
        let mut initializer = None;
        if self.validate(TokenType::EQUAL) {
            initializer = Some(self.expression()?);
        }
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Var(Box::new(Var::new(name, initializer))))
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.validate(TokenType::PRINT) {
            return self.print_statement();
        }
        if self.validate(TokenType::LeftBrace) {
            return Ok(Stmt::Block(Box::new(Block::new(self.block()?))));
        }
        if self.validate(TokenType::IF) {
            return self.if_statement();
        }
        if self.validate(TokenType::WHILE) {
            return self.while_statement();
        }
        if self.validate(TokenType::FOR) {
            return self.for_statement();
        }
        if self.validate(TokenType::BREAK) {
            return self.break_statement();
        }
        if self.validate(TokenType::CONTINUE) {
            return self.continue_statement();
        }
        if self.validate(TokenType::RETURN) {
            return self.return_statement();
        }

        return self.expressions_statement();
    }

    fn return_statement(&mut self) -> Result<Stmt, LoxError> {
        let keyword = self.previous().clone();
        let mut value = None;
        if !self.check(TokenType::SEMICOLON) {
            value = Some(self.expression()?);
        }
        self.consume(TokenType::SEMICOLON, "Expect ';' after return.".to_string())?;
        return Ok(Stmt::Return(Box::new(Return::new(keyword, value))));
    }

    fn break_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::SEMICOLON, "Expect ';' after break.".to_string())?;
        return Ok(Stmt::Break(Box::new(Break::new(self.previous().clone()))));
    }

    fn continue_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after continue.".to_string(),
        )?;
        return Ok(Stmt::Continue(Box::new(Continue::new(
            self.previous().clone(),
        ))));
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut list = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            list.push(self.declaration()?)
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block".to_string())?;
        return Ok(list);
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let val = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
        return Ok(Stmt::Print(Box::new(Print::new(val))));
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.".to_string())?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after if condition.".to_string(),
        )?;

        let body = self.statement()?;
        return Ok(Stmt::While(Box::new(While::new(condition, body))));
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.".to_string())?;

        // Initialization
        let init;
        if self.validate(TokenType::SEMICOLON) {
            init = None;
        } else if self.validate(TokenType::VAR) {
            init = Some(self.val_declaration()?);
        } else {
            init = Some(self.expressions_statement()?);
        }

        // Condition
        let mut condition = None;
        if !self.check(TokenType::SEMICOLON) {
            condition = Some(self.expression()?);
        }
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after loop condition.".to_string(),
        )?;

        //increment
        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(
            TokenType::RightParen,
            "Expect ')' after for clause.".to_string(),
        )?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(Box::new(Block::new(vec![
                body,
                Stmt::Expression(Box::new(Expression::new(increment))),
            ])))
        }

        body = Stmt::While(Box::new(While::new(
            condition.unwrap_or(Expr::Literal(Literal::BOOL(true))),
            body,
        )));

        if let Some(init) = init {
            body = Stmt::Block(Box::new(Block::new(vec![init, body])));
        }

        return Ok(body);
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.".to_string())?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expect ')' after if condition.".to_string(),
        )?;

        let then_branch = self.statement()?;
        let mut else_branch = None;
        if self.validate(TokenType::ELSE) {
            else_branch = Some(self.statement()?);
        }
        return Ok(Stmt::If(Box::new(If::new(
            condition, then_branch, else_branch,
        ))));
    }

    fn expressions_statement(&mut self) -> Result<Stmt, LoxError> {
        let val = self.expression()?;
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.".to_string())?;
        return Ok(Stmt::Expression(Box::new(Expression::new(val))));
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        if self.validate(TokenType::FUN) {
            return self.lambda_expr("lambda");
        }
        self.assignment()
    }

    fn lambda_expr(&mut self, kind: &str) -> Result<Expr, LoxError> {
        let paren = self
            .consume(TokenType::LeftParen, format!("Expect after {} name", kind))?
            .clone();
        let mut params = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() > 255 {
                    return Err(Self::error(
                        self.peek().clone(),
                        "m ax no. of args 255".to_string(),
                    ));
                }
                params.push(
                    self.consume(TokenType::IDENTIFIER, "Expect Param Name".to_string())?
                        .clone(),
                );
                if !self.validate(TokenType::COMMA) {
                    break;
                };
            }
        }
        self.consume(
            TokenType::RightParen,
            "Expect ')' after params".to_string(),
        )?;
        self.consume(TokenType::LeftBrace, "Expect '{' before body".to_string())?;

        let body = self.block()?;
        return Ok(Expr::Lambda(Box::new(Lambda::new(paren, params, body))));
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if self.validate(TokenType::EQUAL) {
            let _equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(Box::new(Assign::new(name.name, value))));
            } else if let Expr::Get(get_expr) = expr {
                return Ok(Expr::Set(Box::new(Set::new(
                    get_expr.object,
                    get_expr.name.clone(),
                    value,
                ))));
            }
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while self.validate(TokenType::OR) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(Logical::new(expr, operator, right)));
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while self.validate(TokenType::AND) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(Logical::new(expr, operator, right)));
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.validate(TokenType::BangEqual) | self.validate(TokenType::EqualEqual) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        return Ok(expr);
    }

    fn validate(&mut self, token: TokenType) -> bool {
        if self.check(token) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.curr += 1
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.curr];
    }
    fn previous(&self) -> &Token {
        return &self.tokens[self.curr - 1];
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.addition()?;

        while self.validate(TokenType::GREATER)
            | self.validate(TokenType::GreaterEqual)
            | self.validate(TokenType::LESS)
            | self.validate(TokenType::LessEqual)
        {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        return Ok(expr);
    }

    fn addition(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.multiplication()?;

        while self.validate(TokenType::MINUS) | self.validate(TokenType::PLUS) {
            let operator = self.previous().clone();
            let right = self.multiplication()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        return Ok(expr);
    }

    fn multiplication(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.validate(TokenType::SLASH) | self.validate(TokenType::STAR) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(Binary::new(expr, operator, right)));
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.validate(TokenType::BANG) | self.validate(TokenType::MINUS) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Box::new(Unary::new(operator, right))));
        }
        return self.call();
    }

    fn call(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.primary()?;

        loop {
            if self.validate(TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else if self.validate(TokenType::DOT) {
                let name = self.consume(
                    TokenType::IDENTIFIER,
                    "Expect property name after '.'.".to_string(),
                )?;
                expr = Expr::Get(Box::new(Get::new(expr, name.clone())));
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, LoxError> {
        let mut args = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if args.len() > 255 {
                    return Err(Self::error(
                        self.peek().clone(),
                        "m ax no. of args 255".to_string(),
                    ));
                }
                args.push(self.expression()?);
                if !self.validate(TokenType::COMMA) {
                    break;
                };
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after args.".to_string())?;
        return Ok(Expr::Call(Box::new(Call::new(callee, paren.clone(), args))));
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.validate(TokenType::THIS) {
            return Ok(Expr::This(Box::new(This::new(self.previous().clone()))));
        }
        if self.validate(TokenType::SUPER) {
            let keyword = self.previous().clone();
            self.consume(TokenType::DOT, "Expect '.' after super.".to_string())?;
            let method = self
                .consume(
                    TokenType::IDENTIFIER,
                    "Expect super class method name.".to_string(),
                )?
                .clone();
            return Ok(Expr::Super(Box::new(Super::new(method, keyword))));
        }
        if self.validate(TokenType::FALSE) {
            return Ok(Expr::Literal(Literal::BOOL(false)));
        }
        if self.validate(TokenType::TRUE) {
            return Ok(Expr::Literal(Literal::BOOL(true)));
        }
        if self.validate(TokenType::NIL) {
            return Ok(Expr::Literal(Literal::NIL));
        }

        if self.validate(TokenType::NUMBER) || self.validate(TokenType::STRING) {
            return Ok(Expr::Literal(self.previous().literal.clone().unwrap()));
        }

        if self.validate(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            )?;
            return Ok(Expr::Grouping(Box::new(Grouping::new(expr))));
        }

        if self.validate(TokenType::IDENTIFIER) {
            return Ok(Expr::Variable(Box::new(Variable::new(
                self.previous().clone(),
            ))));
        }

        return Err(Self::error(
            self.peek().clone(),
            "Expect Expression".to_string(),
        ));
    }

    fn consume(&mut self, token: TokenType, message: String) -> Result<&Token, LoxError> {
        return if self.check(token) {
            Ok(self.advance())
        } else {
            let err_token = self.peek();
            Err(Self::error(err_token.clone(), message))
        }
    }

    fn error(token: Token, _message: String) -> LoxError {
        return LoxError::ParserError(token.lexeme.clone(), token.line_no);
    }

    fn synchronize(&mut self) {
        use crate::token_type::TokenType::*;

        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == SEMICOLON {
                return;
            };
            match self.peek().token_type {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => {}
            };
            self.advance();
        }
    }
}
