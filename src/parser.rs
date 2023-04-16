use crate::ast::*;
use crate::token::{Precedence, Token, TokenKind};
use std::fmt::{self, Display};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    pub tokens: Peekable<IntoIter<Token>>,
    pub current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            current_token: Token::new(TokenKind::EOF, String::from("")),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.current_token = self.tokens.next()?;
        Some(self.current_token.clone())
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    pub fn parse(&mut self) -> Program {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while let Some(_) = self.peek() {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.peek() {
            Some(token) => match token.kind {
                TokenKind::LET => self.parse_let_statement(),
                TokenKind::RETURN => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            },
            None => None,
        }
    }

    // let <identifier> = <expression>;
    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let let_token = self.next()?;

        if !self.expect_peek(TokenKind::IDENTIFIER) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenKind::ASSIGN) {
            return None;
        }

        // parse_expression
        let dummy_expression = Box::new(Identifier {
            token: let_token.clone(),
            value: String::from("dummy"),
        });

        while !self.current_token_is(TokenKind::SEMICOLON) {
            self.next();
        }

        Some(Box::new(LetStatement {
            token: let_token,
            name,
            value: dummy_expression,
        }))
    }

    // return <expression>;
    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let return_token = self.next()?;

        // TODO: Expressionを実装後に書き換える
        self.next();

        while !self.current_token_is(TokenKind::SEMICOLON) {
            self.next();
        }

        // TODO: Expressionを実装後に書き換える
        let dummy_expression = Box::new(Identifier {
            token: return_token.clone(),
            value: String::from("dummy"),
        });

        Some(Box::new(ReturnStatement {
            token: return_token.clone(),
            return_value: dummy_expression,
        }))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let exp_token = self.next()?;

        let expression = self.parse_expression(Precedence::Lowest);

        while self.peek_token_is(TokenKind::SEMICOLON) {
            self.next();
        }

        return Some(Box::new(ExpressionStatement {
            token: exp_token.clone(),
            expression: expression.unwrap(),
        }));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let mut left_exp = match self.current_token.kind {
            TokenKind::IDENTIFIER => self.parse_identifier(),
            TokenKind::INTEGER => self.parse_integer_literal(),
            TokenKind::BANG => self.parse_prefix_expression(),
            TokenKind::MINUS => self.parse_prefix_expression(),
            _ => None,
        };

        while !self.peek_token_is(TokenKind::SEMICOLON) && precedence < self.peek_precedence() {
            let token = self.peek()?;

            match token.kind {
                TokenKind::PLUS
                | TokenKind::MINUS
                | TokenKind::SLASH
                | TokenKind::ASTERISK
                | TokenKind::EQ
                | TokenKind::NOT_EQ
                | TokenKind::LT
                | TokenKind::GT => {
                    self.next();
                    left_exp = self.parse_infix_expression(left_exp.unwrap());
                }
                _ => break,
            };
        }

        left_exp
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<dyn Expression>> {
        let prefix_token = self.current_token.clone();

        self.next();

        let right_expression = self.parse_expression(Precedence::Prefix);

        Some(Box::new(PrefixExpression {
            token: prefix_token.clone(),
            operator: prefix_token.literal,
            right: right_expression.unwrap(),
        }))
    }

    fn parse_infix_expression(
        &mut self,
        left_exp: Box<dyn Expression>,
    ) -> Option<Box<dyn Expression>> {
        let infix_token = self.current_token.clone();
        let precedence = self.current_precedence();

        self.next();

        let right_expression = self.parse_expression(precedence);

        Some(Box::new(InfixExpression {
            token: infix_token.clone(),
            operator: infix_token.literal,
            left: left_exp,
            right: right_expression.unwrap(),
        }))
    }

    fn parse_identifier(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Box<dyn Expression>> {
        Some(Box::new(IntegerLiteral {
            token: self.current_token.clone(),
            value: self.current_token.literal.parse().unwrap(),
        }))
    }

    fn peek_token_is(&mut self, kind: TokenKind) -> bool {
        self.peek().map_or(false, |token| token.kind == kind)
    }

    fn current_token_is(&mut self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind.clone()) {
            self.next();
            true
        } else {
            eprintln!(
                "expected next token to be {:?}, got {:?} instead",
                kind,
                self.peek()
            );
            false
        }
    }

    fn peek_precedence(&mut self) -> Precedence {
        match self.peek() {
            Some(token) => token.precedence(),
            None => Precedence::Lowest,
        }
    }

    fn current_precedence(&mut self) -> Precedence {
        self.current_token.precedence()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parser() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
                "#;

        let lexer = tokenize(input);

        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        for stmt in program.statements {
            assert_eq!(stmt.token_literal(), "let");
        }
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
                "#;

        let lexer = tokenize(input);

        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        for stmt in program.statements {
            assert_eq!(stmt.token_literal(), "return");
        }
    }

    #[test]
    fn test_expression_statement() {
        let input = r#"
        hoge;
        fuga;
                "#;

        let lexer = tokenize(input);

        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        assert_eq!(program.statements[0].token_literal(), "hoge");
        assert_eq!(program.statements[1].token_literal(), "fuga");
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = tokenize(input);

        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        assert_eq!(program.statements[0].token_literal(), "5");
    }

    #[test]
    fn test_prefix_expression() {
        let input = r#"
    !5;
    -15;
            "#;

        let lexer = tokenize(input);

        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        assert_eq!(program.statements[0].string(), "(!5)");
        assert_eq!(program.statements[1].string(), "(-15)");
    }

    #[test]
    fn test_infix_expression() {
        let input = r#"
5 + 5;
5 - 5;
5 * 5;
5 / 5;
5 > 5;
5 < 5;
5 == 5;
5 != 5;
-a * b;
!-a;
a + b + c;
a + b - c;
a * b * c;
a * b / c;
a + b / c;
a + b * c + d / e - f;
3 + 4; -5 * 5;
5 > 4 == 3 < 4;
3 + 4 * 5 == 3 * 1 + 4 * 5;
"#;
        let lexer = tokenize(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        assert_eq!(program.statements[0].string(), "(5 + 5)");
        assert_eq!(program.statements[1].string(), "(5 - 5)");
        assert_eq!(program.statements[2].string(), "(5 * 5)");
        assert_eq!(program.statements[3].string(), "(5 / 5)");
        assert_eq!(program.statements[4].string(), "(5 > 5)");
        assert_eq!(program.statements[5].string(), "(5 < 5)");
        assert_eq!(program.statements[6].string(), "(5 == 5)");
        assert_eq!(program.statements[7].string(), "(5 != 5)");
        assert_eq!(program.statements[8].string(), "((-a) * b)");
        assert_eq!(program.statements[9].string(), "(!(-a))");
        assert_eq!(program.statements[10].string(), "((a + b) + c)");
        assert_eq!(program.statements[11].string(), "((a + b) - c)");
        assert_eq!(program.statements[12].string(), "((a * b) * c)");
        assert_eq!(program.statements[13].string(), "((a * b) / c)");
        assert_eq!(program.statements[14].string(), "(a + (b / c))");
        assert_eq!(
            program.statements[15].string(),
            "(((a + (b * c)) + (d / e)) - f)"
        );
        assert_eq!(program.statements[16].string(), "(3 + 4)");
        assert_eq!(program.statements[17].string(), "((-5) * 5)");
        assert_eq!(program.statements[18].string(), "((5 > 4) == (3 < 4))");
        assert_eq!(
            program.statements[19].string(),
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"
        );
    }
}
