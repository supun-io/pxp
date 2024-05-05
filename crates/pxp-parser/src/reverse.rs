use pxp_ast::{
    literals::LiteralKind, operators::AssignmentOperationExpression, variables::Variable,
    Expression, ExpressionKind, Statement, StatementKind,
};
use pxp_symbol::SymbolTable;
use pxp_token::TokenKind;

use crate::parse;

struct Reverse {
    line: usize,
    column: usize,
}

impl Reverse {
    pub fn new() -> Self {
        Self { line: 1, column: 0 }
    }

    pub fn parse(&mut self, statements: Vec<Statement>) -> String {
        let mut result = String::new();
        println!("{:?}", statements);
        for stmt in statements.iter() {
            let code = self.stmt_to_code(stmt);
            result.push_str(&code);
        }
        result
    }

    fn stmt_to_code(&mut self, stmt: &Statement) -> String {
        let mut result = String::new();

        if stmt.span.start.line > self.line {
            result.push_str("\n");
            self.line = stmt.span.start.line;
            self.column = 0;
        }

        let val = match stmt.kind {
            StatementKind::FullOpeningTag(_) => "<?php".to_string(),
            StatementKind::ShortOpeningTag(_) => "<?".to_string(),
            StatementKind::EchoOpeningTag(_) => "<?=".to_string(),
            StatementKind::ClosingTag(_) => "?>".to_string(),
            StatementKind::InlineHtml(ref html) => match html.html.symbol {
                Some(val) => val.to_string(),
                _ => "".to_string(),
            },
            StatementKind::Echo(ref echo) => {
                let mut result = String::new();
                result.push_str("echo ");
                for expr in echo.values.iter() {
                    result.push_str(&self.expr_to_code(expr));
                }
                result.push_str(";"); // todo: consider echo.ending
                result
            }
            StatementKind::Expression(ref expr) => self.expr_to_code(&expr.expression),
            _ => format!("{:?}", stmt.kind),
            // _ => todo!(),
        };

        result.push_str(&val);

        self.column = val.len() + self.column;

        if stmt.span.end.column > self.column {
            result.push_str(&" ".repeat(stmt.span.end.column - self.column));
            self.column = stmt.span.end.column;
        }

        result
    }

    fn expr_to_code(&mut self, expr: &Expression) -> String {
        let mut result = String::new();

        if expr.span.start.line > self.line {
            result.push_str("\n");
            self.line = expr.span.start.line;
            self.column = 0;
        }

        let val = match expr.kind {
            ExpressionKind::Literal(ref literal) => {
                if let Some(symbol) = literal.token.symbol {
                    symbol.to_string()
                } else {
                    String::new()
                }
            }
            ExpressionKind::Variable(ref var) => match var {
                Variable::SimpleVariable(simple) => simple.symbol.to_string(),
                _ => todo!(),
            },
            ExpressionKind::AssignmentOperation(ref assignment_expr) => match assignment_expr {
                AssignmentOperationExpression::Assign {
                    left,
                    right,
                    equals: _,
                } => {
                    let mut result = String::new();
                    result.push_str(&self.expr_to_code(&left));
                    result.push_str(" = ");
                    result.push_str(&self.expr_to_code(&right));
                    result
                }
                _ => todo!(),
            },
            _ => format!("{:?}", expr.kind),
        };

        result.push_str(&val);

        self.column = val.len() + self.column;

        if expr.span.end.column > self.column {
            result.push_str(&" ".repeat(expr.span.end.column - self.column));
            self.column = expr.span.end.column;
        }

        result
    }
}

fn reverse(stmts: Vec<Statement>) -> String {
    let mut reverse = Reverse::new();
    reverse.parse(stmts)
}

#[test]
fn reverse_test() {
    let mut symbol_table = SymbolTable::the();
//     let output = parse(
//         &"<html><?php  echo 'Hello, World!';
// $foo = 'bar';?></html>",
//         &mut symbol_table,
//     );
//
    let output = parse(&"<html><?php ?>" , &mut symbol_table);
    let reversed = reverse(output.ast);
    println!("{:?}", reversed);
}
