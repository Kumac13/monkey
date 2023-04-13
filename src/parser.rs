use crate::ast::{
    Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Precedence, Program,
    ReturnStatement, Statement,
};
use crate::token::{Token, TokenKind};
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

        let expression = self.parse_expression(Precedence::LOWEST);

        while self.peek_token_is(TokenKind::SEMICOLON) {
            self.next();
        }

        return Some(Box::new(ExpressionStatement {
            token: exp_token.clone(),
            expression: expression.unwrap(),
        }));
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<dyn Expression>> {
        let mut exp = match self.current_token.kind {
            TokenKind::IDENTIFIER => self.parse_identifier(),
            TokenKind::INTEGER => self.parse_integer_literal(),
            _ => None,
        };

        self.next();

        exp
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
foobar;
        "#;

        let lexer = tokenize(input);

        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        assert_eq!(program.statements[0].token_literal(), "foobar");
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = tokenize(input);

        let mut parser = Parser::new(lexer);
        let program = parser.parse();

        assert_eq!(program.statements[0].token_literal(), "5");
    }
}
