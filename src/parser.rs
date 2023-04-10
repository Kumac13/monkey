use crate::ast::{Identifier, LetStatement, Program, Statement};
use crate::token::{Token, TokenKind};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
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
                println!("statement: {:?}", statement);
                program.statements.push(statement);
            }
            self.next();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.peek() {
            Some(token) => match token.kind {
                TokenKind::LET => self.parse_let_statement(),
                _ => None,
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

        let name_token = self.next()?;
        let name = Identifier {
            token: name_token.clone(),
            value: name_token.literal.clone(),
        };

        if !self.expect_peek(TokenKind::ASSIGN) {
            return None;
        }

        if let Some(t) = self.peek() {
            if t.kind == TokenKind::SEMICOLON {
                self.next();
            }
        }

        let dummy_expression = Box::new(Identifier {
            token: let_token.clone(),
            value: String::from("dummy"),
        });

        Some(Box::new(LetStatement {
            token: let_token.clone(),
            name,
            value: dummy_expression,
        }))
    }

    fn peek_token_is(&mut self, kind: TokenKind) -> bool {
        self.peek().map_or(false, |token| token.kind == kind)
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind) {
            true
        } else {
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

        assert_eq!(program.statements.len(), 3);
    }
}
