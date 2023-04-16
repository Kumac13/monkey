use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone, Hash)]
pub enum TokenKind {
    ILLEGAL,
    EOF,
    INTEGER,
    IDENTIFIER,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOT_EQ,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::ILLEGAL => "ILLEGAL",
            TokenKind::EOF => "EOF",
            TokenKind::INTEGER => "INTEGER",
            TokenKind::IDENTIFIER => "IDENTIFIER",
            TokenKind::ASSIGN => "=",
            TokenKind::PLUS => "+",
            TokenKind::MINUS => "-",
            TokenKind::BANG => "!",
            TokenKind::ASTERISK => "*",
            TokenKind::SLASH => "/",
            TokenKind::LT => "<",
            TokenKind::GT => ">",
            TokenKind::EQ => "==",
            TokenKind::NOT_EQ => "!=",
            TokenKind::COMMA => ",",
            TokenKind::SEMICOLON => ";",
            TokenKind::LPAREN => "(",
            TokenKind::RPAREN => ")",
            TokenKind::LBRACE => "{",
            TokenKind::RBRACE => "}",
            TokenKind::FUNCTION => "FUNCTION",
            TokenKind::LET => "LET",
            TokenKind::TRUE => "TRUE",
            TokenKind::FALSE => "FALSE",
            TokenKind::IF => "IF",
            TokenKind::ELSE => "ELSE",
            TokenKind::RETURN => "RETURN",
        };
        write!(f, "{}", s)
    }
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Token {
        Token { kind, literal }
    }

    pub fn precedence(&self) -> Precedence {
        match self.kind {
            TokenKind::EQ | TokenKind::NOT_EQ => Precedence::Equals,
            TokenKind::LT | TokenKind::GT => Precedence::Lessgreater,
            TokenKind::PLUS | TokenKind::MINUS => Precedence::Sum,
            TokenKind::SLASH | TokenKind::ASTERISK => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token{{ kind: {}, literal: {} }}",
            self.kind, self.literal
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 1,
    Equals = 2,      // ==
    Lessgreater = 3, // > or <
    Sum = 4,         // +
    Product = 5,     // *
    Prefix = 6,      // -X or !X
    Call = 7,        // my_function(x){}
}
