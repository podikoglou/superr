use qua_ast::ast::{
    expr::{BinaryExpr, Expr, UnaryExpr},
    literal::Literal,
    program::{Function, Parameter, Program},
    stmt::{Stmt, Type},
};
use qua_lexer::{keyword::Keyword, lexer::Lexer, token::Token};

use crate::error::{ParseError, ParseResult};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    pub fn parse_program(&mut self) -> ParseResult<Program> {
        let mut functions = Vec::new();

        while !self.is_at_end() {
            functions.push(self.parse_function()?);
        }

        Ok(Program { functions })
    }

    fn parse_function(&mut self) -> ParseResult<Function> {
        // Parse return type (if present)
        let return_type = if self.current_token_is_type() {
            let type_token = self.current_token.clone();
            self.next_token();
            Some(self.token_to_type(&type_token)?)
        } else {
            None
        };

        // Parse function name
        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "function name".to_string(),
                    found: self.current_token.clone(),
                });
            }
        };
        self.next_token();

        // Parse parameters
        self.expect_token(&Token::OpenParen)?;

        let mut parameters = Vec::new();

        if !self.current_token_is(&Token::CloseParen) {
            loop {
                parameters.push(self.parse_parameter()?);

                if self.current_token_is(&Token::Comma) {
                    self.next_token();
                } else {
                    break;
                }
            }
        }

        self.expect_token(&Token::CloseParen)?;

        // Parse function body
        let body = self.parse_statement()?;

        Ok(Function {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_parameter(&mut self) -> ParseResult<Parameter> {
        // Parse parameter type
        let type_token = self.current_token.clone();

        if !self.current_token_is_type() {
            return Err(ParseError::UnexpectedToken {
                expected: "type".to_string(),
                found: type_token,
            });
        }

        let type_name = self.token_to_type(&type_token)?;

        self.next_token();

        // Parse parameter name
        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "parameter name".to_string(),
                    found: self.current_token.clone(),
                });
            }
        };
        self.next_token();

        Ok(Parameter { name, type_name })
    }

    fn parse_statement(&mut self) -> ParseResult<Stmt> {
        match &self.current_token {
            Token::OpenBrace => self.parse_block_statement(),
            Token::Keyword(Keyword::Return) => self.parse_return_statement(),
            Token::Keyword(Keyword::If) => self.parse_if_statement(),
            Token::Keyword(Keyword::While) => self.parse_while_statement(),
            Token::Keyword(Keyword::For) => self.parse_for_statement(),
            Token::Keyword(Keyword::Break) => {
                self.next_token();
                self.expect_token(&Token::Semicolon)?;
                Ok(Stmt::Break)
            }
            Token::Keyword(Keyword::Continue) => {
                self.next_token();
                self.expect_token(&Token::Semicolon)?;
                Ok(Stmt::Continue)
            }
            _ => {
                // Check if it's a variable declaration or assignment
                if self.current_token_is_type() && self.peek_token_is_identifier() {
                    self.parse_var_declaration()
                } else if self.current_token_is_identifier() && self.peek_token_is(&Token::Equals) {
                    self.parse_assignment()
                } else {
                    // Expression statement
                    let expr = self.parse_expression()?;
                    self.expect_token(&Token::Semicolon)?;
                    Ok(Stmt::Expression(expr))
                }
            }
        }
    }

    fn parse_block_statement(&mut self) -> ParseResult<Stmt> {
        self.expect_token(&Token::OpenBrace)?;
        let mut statements = Vec::new();

        while !self.current_token_is(&Token::CloseBrace) && !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        self.expect_token(&Token::CloseBrace)?;
        Ok(Stmt::Block(statements))
    }

    fn parse_return_statement(&mut self) -> ParseResult<Stmt> {
        self.next_token(); // consume 'return'

        let expr = if self.current_token_is(&Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect_token(&Token::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_if_statement(&mut self) -> ParseResult<Stmt> {
        self.next_token(); // consume 'if'

        self.expect_token(&Token::OpenParen)?;

        let condition = self.parse_expression()?;

        self.expect_token(&Token::CloseParen)?;

        let then_branch = Box::new(self.parse_statement()?);

        let else_branch = if self.current_token_is(&Token::Keyword(Keyword::Else)) {
            self.next_token();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> ParseResult<Stmt> {
        self.next_token(); // consume 'while'

        self.expect_token(&Token::OpenParen)?;

        let condition = self.parse_expression()?;

        self.expect_token(&Token::CloseParen)?;

        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::While { condition, body })
    }

    fn parse_for_statement(&mut self) -> ParseResult<Stmt> {
        self.next_token(); // consume 'for'

        self.expect_token(&Token::OpenParen)?;

        // Parse variable type (optional)
        let var_type = if self.current_token_is_type() {
            let type_token = self.current_token.clone();
            self.next_token();
            Some(self.token_to_type(&type_token)?.to_string())
        } else {
            None
        };

        // Parse variable name
        let var_name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "variable name".to_string(),
                    found: self.current_token.clone(),
                });
            }
        };
        self.next_token();

        self.expect_token(&Token::Keyword(Keyword::In))?;
        let iterable = self.parse_expression()?;
        self.expect_token(&Token::CloseParen)?;

        let body = Box::new(self.parse_statement()?);

        Ok(Stmt::For {
            var_name,
            var_type,
            iterable,
            body,
        })
    }

    fn parse_var_declaration(&mut self) -> ParseResult<Stmt> {
        // Parse type
        let type_token = self.current_token.clone();
        let type_name = Some(self.token_to_type(&type_token)?.to_string());

        self.next_token();

        // Parse variable name
        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "variable name".to_string(),
                    found: self.current_token.clone(),
                });
            }
        };

        self.next_token();

        // Parse optional initialization
        let value = if self.current_token_is(&Token::Equals) {
            self.next_token();
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_token(&Token::Semicolon)?;

        Ok(Stmt::VarDecl {
            name,
            type_name,
            value,
        })
    }

    fn parse_assignment(&mut self) -> ParseResult<Stmt> {
        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "variable name".to_string(),
                    found: self.current_token.clone(),
                });
            }
        };

        self.next_token();

        self.expect_token(&Token::Equals)?;

        let value = self.parse_expression()?;

        self.expect_token(&Token::Semicolon)?;

        Ok(Stmt::Assignment { name, value })
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_logical_and()?;

        while self.current_token_is(&Token::Or) {
            self.next_token();
            let right = self.parse_logical_and()?;
            expr = Expr::Binary(BinaryExpr::Or(Box::new(expr), Box::new(right)));
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_equality()?;

        while self.current_token_is(&Token::And) {
            self.next_token();
            let right = self.parse_equality()?;
            expr = Expr::Binary(BinaryExpr::And(Box::new(expr), Box::new(right)));
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_comparison()?;

        while matches!(self.current_token, Token::EqualsEquals | Token::NotEquals) {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.parse_comparison()?;

            expr = match op {
                Token::EqualsEquals => {
                    Expr::Binary(BinaryExpr::Equals(Box::new(expr), Box::new(right)))
                }
                Token::NotEquals => {
                    Expr::Binary(BinaryExpr::NotEquals(Box::new(expr), Box::new(right)))
                }
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_term()?;

        while matches!(
            self.current_token,
            Token::Greater | Token::GreaterEq | Token::Lesser | Token::LesserEq
        ) {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.parse_term()?;

            expr = match op {
                Token::Greater => {
                    Expr::Binary(BinaryExpr::Greater(Box::new(expr), Box::new(right)))
                }
                Token::GreaterEq => {
                    Expr::Binary(BinaryExpr::GreaterEqual(Box::new(expr), Box::new(right)))
                }
                Token::Lesser => Expr::Binary(BinaryExpr::Less(Box::new(expr), Box::new(right))),
                Token::LesserEq => {
                    Expr::Binary(BinaryExpr::LessEqual(Box::new(expr), Box::new(right)))
                }
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_factor()?;

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.parse_factor()?;

            expr = match op {
                Token::Plus => Expr::Binary(BinaryExpr::Add(Box::new(expr), Box::new(right))),
                Token::Minus => Expr::Binary(BinaryExpr::Subtract(Box::new(expr), Box::new(right))),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_unary()?;

        while matches!(
            self.current_token,
            Token::Asterisk | Token::Slash | Token::Percent
        ) {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.parse_unary()?;

            expr = match op {
                Token::Asterisk => {
                    Expr::Binary(BinaryExpr::Multiply(Box::new(expr), Box::new(right)))
                }
                Token::Slash => Expr::Binary(BinaryExpr::Divide(Box::new(expr), Box::new(right))),
                Token::Percent => Expr::Binary(BinaryExpr::Modulo(Box::new(expr), Box::new(right))),
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> ParseResult<Expr> {
        match &self.current_token {
            Token::Not => {
                self.next_token();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryExpr::Not(Box::new(expr))))
            }
            Token::Minus => {
                self.next_token();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryExpr::Minus(Box::new(expr))))
            }
            Token::Plus => {
                self.next_token();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnaryExpr::Plus(Box::new(expr))))
            }
            _ => self.parse_call(),
        }
    }

    fn parse_call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_primary()?;

        while self.current_token_is(&Token::OpenParen) {
            self.next_token();
            let mut args = Vec::new();

            if !self.current_token_is(&Token::CloseParen) {
                loop {
                    args.push(self.parse_expression()?);

                    if self.current_token_is(&Token::Comma) {
                        self.next_token();
                    } else {
                        break;
                    }
                }
            }

            self.expect_token(&Token::CloseParen)?;

            expr = match expr {
                Expr::Identifier(name) => Expr::Call { name, args },
                _ => {
                    return Err(ParseError::InvalidExpression(
                        "Can only call functions".to_string(),
                    ));
                }
            };
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> ParseResult<Expr> {
        match &self.current_token.clone() {
            Token::IntLiteral(value) => {
                let value = *value;
                self.next_token();
                Ok(Expr::Literal(Literal::Int(value)))
            }
            Token::FloatLiteral(value) => {
                let value = *value;
                self.next_token();
                Ok(Expr::Literal(Literal::Float(value)))
            }
            Token::StringLiteral(value) => {
                let value = value.clone();
                self.next_token();
                Ok(Expr::Literal(Literal::String(value)))
            }
            Token::CharLiteral(value) => {
                let value = *value;
                self.next_token();
                Ok(Expr::Literal(Literal::Char(value)))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.next_token();
                Ok(Expr::Identifier(name))
            }
            Token::OpenParen => {
                self.next_token();
                let expr = self.parse_expression()?;
                self.expect_token(&Token::CloseParen)?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: self.current_token.clone(),
            }),
        }
    }

    // Helper methods
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.current_token) == std::mem::discriminant(token)
    }

    fn peek_token_is(&self, token: &Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(token)
    }

    fn current_token_is_identifier(&self) -> bool {
        matches!(self.current_token, Token::Identifier(_))
    }

    fn peek_token_is_identifier(&self) -> bool {
        matches!(self.peek_token, Token::Identifier(_))
    }

    fn current_token_is_type(&self) -> bool {
        matches!(self.current_token, Token::Identifier(ref name) if name == "int" || name == "float" || name == "string" || name == "char")
    }

    fn token_to_type(&self, token: &Token) -> ParseResult<Type> {
        match token {
            Token::Identifier(name) => Ok(Type::from(name.as_str())),
            _ => Err(ParseError::UnexpectedToken {
                expected: "type".to_string(),
                found: token.clone(),
            }),
        }
    }

    fn expect_token(&mut self, expected: &Token) -> ParseResult<()> {
        if self.current_token_is(expected) {
            self.next_token();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: self.current_token.clone(),
            })
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current_token, Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qua_lexer::lexer::Lexer;

    #[test]
    fn test_parse_simple_expression() {
        let source = "2 + 3 * 4";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);

        let expr = parser.parse_expression().unwrap();

        // Should parse as: 2 + (3 * 4)
        match expr {
            Expr::Binary(BinaryExpr::Add(left, right)) => {
                assert!(matches!(left.as_ref(), Expr::Literal(Literal::Int(2))));
                assert!(matches!(
                    right.as_ref(),
                    Expr::Binary(BinaryExpr::Multiply(_, _))
                ));
            }
            _ => panic!("Expected binary addition"),
        }
    }

    #[test]
    fn test_parse_function_call() {
        let source = "factorial(5)";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);

        let expr = parser.parse_expression().unwrap();

        match expr {
            Expr::Call { name, args } => {
                assert_eq!(name, "factorial");
                assert_eq!(args.len(), 1);
                assert!(matches!(args[0], Expr::Literal(Literal::Int(5))));
            }
            _ => panic!("Expected function call"),
        }
    }

    #[test]
    fn test_parse_var_declaration() {
        let source = "int x = 42;";
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);

        let stmt = parser.parse_statement().unwrap();

        match stmt {
            Stmt::VarDecl {
                name,
                type_name,
                value,
            } => {
                assert_eq!(name, "x");
                assert_eq!(type_name, Some("int".to_string()));
                assert!(matches!(value, Some(Expr::Literal(Literal::Int(42)))));
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_factorial_function() {
        let source = r#"
int factorial(int x) {
    int y = 1;

    for(int i in range(2, x + 1)) {
        y = y * i;
    }

    return y;
}
"#;
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();

        assert_eq!(program.functions.len(), 1);

        let function = &program.functions[0];
        assert_eq!(function.name, "factorial");
        assert_eq!(function.parameters.len(), 1);
        assert_eq!(function.parameters[0].name, "x");
        assert!(matches!(function.return_type, Some(Type::Int)));

        // The body should be a block
        assert!(matches!(function.body, Stmt::Block(_)));
    }
}
