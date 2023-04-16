use crate::token::Token;
use std::fmt::Debug;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node + Debug {
    fn statement_node(&self);
}

pub trait Expression: Node + Debug {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            "".to_string()
        }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            out += &statement.string();
        }
        out
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Box<dyn Expression>,
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.token_literal() + " " + &self.name.string() + " = " + &self.value.string() + ";"
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Box<dyn Expression>,
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn string(&self) -> String {
        self.token_literal() + " " + &self.return_value.string() + ";"
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Box<dyn Expression>,
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.expression.string()
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Expression for Identifier {
    fn expression_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Expression for Boolean {
    fn expression_node(&self) {}
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out += "(";
        out += &self.operator;
        out += &self.right.string();
        out += ")";
        out
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut out = String::new();
        out += "(";
        out += &self.left.string();
        out += " ";
        out += &self.operator;
        out += " ";
        out += &self.right.string();
        out += ")";
        out
    }
}
