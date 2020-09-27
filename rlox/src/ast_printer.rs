// use crate::grammar::{Expr::*, Visitor, Stmt::*};
// use crate::scanner::*;

// pub struct ASTprinter();

// impl Visitor<String> for ASTprinter {
//     fn visitBinaryExpr(&mut self, val: &mut Binary) -> Result<String, LoxError> { 
//         Ok(format!("(binary {} {:?} {})", val.left.accept(self)?, val.operator.tokenType, val.right.accept(self)?))
//     }

//     fn visitGroupingExpr(&mut self, val: &mut Grouping) -> Result<String, LoxError> { 
//         Ok(format!("(grouping {})", val.expression.accept(self)?))    
//     }

//     fn visitLiteralExpr(&mut self, val: &mut Literal) -> Result<String, LoxError> { 
//         Ok(match val {
//             Literal::BOOL(v) => format!("(literal {})", v),
//             Literal::STRING(v) => format!("(literal {})", v),
//             Literal::NUM(v)  => format!("(literal {})", v),
//             Literal::NIL => format!("(literal NIL)"),
//         })    
//     }
    
//     fn visitUnaryExpr(&mut self, val: &mut Unary) -> Result<String, LoxError> { 
//         Ok(format!("(unary {:?} {})", val.operator.tokenType, val.right.accept(self)?))    
//     }
//     fn visitPrintStmt(&mut self, val : &mut Print) -> Result<String, LoxError> { 
//         Ok(format!("(print {})", val.expr.accept(self)?))
//     }
//     fn visitExpressionStmt(&mut self, val : &mut Expression) -> Result<String, LoxError> { 
//         Ok(format!("(stmt {})", val.expr.accept(self)?))
//     }
// fn visitVarStmt(&mut self, val: &mut Var) -> Result<String, LoxError> { todo!() }
// fn visitVariableStmt(&mut self, val: &mut Variable) -> Result<String, LoxError> { todo!() }
// fn visitAssignStmt(&mut self, val: &mut Assign) -> Result<String, LoxError> { todo!() }
// fn visitBlockStmt(&mut self, val: &mut Block) -> Result<String, LoxError> { todo!() }
// }

// pub fn test() {
//     let mut sample = Expr::Binary(Box::new(Binary::new(
//         Expr::Unary(Box::new(Unary::new(
//             Token::new(TokenType::MINUS, 0, None, empty_str), 
//             Expr::Literal(Literal::NUM(34.5))
//         ))), 
//         Token::new(TokenType::STAR, 0,None, empty_str), 
//         Expr::Grouping(Box::new(Grouping::new(
//             Expr::Literal(Literal::NUM(12.5))
//         ))
//     ))));
//     let mut astPrinter = ASTprinter{};
//     match sample.accept(&mut astPrinter) {
//         Ok(val) => println!("{}", val),
//         Err(err) => println!("{:?}", err), 
//     }
// }
