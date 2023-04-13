use crate::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str::Chars;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut input = input.chars().peekable();

    while let Some(c) = input.next() {
        if c.is_whitespace() {
            continue;
        }
        match c {
            '=' => {
                if let Some(c) = input.peek() {
                    if *c == '=' {
                        tokens.push(Token::new(TokenKind::EQ, String::from("==")));
                        input.next();
                    } else {
                        tokens.push(Token::new(TokenKind::ASSIGN, String::from("=")));
                    }
                }
            }
            '+' => tokens.push(Token::new(TokenKind::PLUS, String::from("+"))),
            '-' => tokens.push(Token::new(TokenKind::MINUS, String::from("-"))),
            '!' => {
                if let Some(c) = input.peek() {
                    if *c == '=' {
                        tokens.push(Token::new(TokenKind::NOT_EQ, String::from("!=")));
                        input.next();
                    } else {
                        tokens.push(Token::new(TokenKind::BANG, String::from("!")));
                    }
                }
            }
            '*' => tokens.push(Token::new(TokenKind::ASTERISK, String::from("*"))),
            '/' => tokens.push(Token::new(TokenKind::SLASH, String::from("/"))),
            '<' => tokens.push(Token::new(TokenKind::LT, String::from("<"))),
            '>' => tokens.push(Token::new(TokenKind::GT, String::from(">"))),
            ',' => tokens.push(Token::new(TokenKind::COMMA, String::from(","))),
            ';' => tokens.push(Token::new(TokenKind::SEMICOLON, String::from(";"))),
            '(' => tokens.push(Token::new(TokenKind::LPAREN, String::from("("))),
            ')' => tokens.push(Token::new(TokenKind::RPAREN, String::from(")"))),
            '{' => tokens.push(Token::new(TokenKind::LBRACE, String::from("{"))),
            '}' => tokens.push(Token::new(TokenKind::RBRACE, String::from("}"))),
            _ => {
                if c.is_ascii_digit() {
                    tokens.push(Token::new(
                        TokenKind::INTEGER,
                        consume_integer(&mut input, c),
                    ));
                } else if is_literal(c) {
                    let literal = consume_literal(&mut input, c);
                    tokens.push(search_keywords(&literal));
                } else {
                    tokens.push(Token::new(TokenKind::ILLEGAL, String::from(c)));
                }
            }
        }
    }
    tokens
}

fn consume_integer(input: &mut Peekable<Chars>, current_c: char) -> String {
    let mut num = String::from(current_c);
    while let Some(c) = input.peek() {
        if c.is_ascii_digit() {
            num.push(*c);
            input.next();
        } else {
            break;
        }
    }
    num
}

fn consume_literal(input: &mut Peekable<Chars>, current_c: char) -> String {
    let mut literal = String::from(current_c);
    while let Some(c) = input.peek() {
        if is_literal(*c) {
            literal.push(*c);
            input.next();
        } else {
            break;
        }
    }
    literal
}

fn is_literal(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit()
}

fn search_keywords(literal: &str) -> Token {
    match literal {
        "fn" => Token::new(TokenKind::FUNCTION, literal.to_string()),
        "let" => Token::new(TokenKind::LET, literal.to_string()),
        "true" => Token::new(TokenKind::TRUE, literal.to_string()),
        "false" => Token::new(TokenKind::FALSE, literal.to_string()),
        "if" => Token::new(TokenKind::IF, literal.to_string()),
        "else" => Token::new(TokenKind::ELSE, literal.to_string()),
        "return" => Token::new(TokenKind::RETURN, literal.to_string()),
        _ => Token::new(TokenKind::IDENTIFIER, literal.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_tokenize() {
        let input = r#"
let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if ( 5 < 10 ) {
  return true;
} else {
  return false;
}

10 == 10;
10 != 9;
        "#;
        let tokens = tokenize(input);

        let expected = vec![
            Token::new(TokenKind::LET, String::from("let")),
            Token::new(TokenKind::IDENTIFIER, String::from("five")),
            Token::new(TokenKind::ASSIGN, String::from("=")),
            Token::new(TokenKind::INTEGER, String::from("5")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::LET, String::from("let")),
            Token::new(TokenKind::IDENTIFIER, String::from("ten")),
            Token::new(TokenKind::ASSIGN, String::from("=")),
            Token::new(TokenKind::INTEGER, String::from("10")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::LET, String::from("let")),
            Token::new(TokenKind::IDENTIFIER, String::from("add")),
            Token::new(TokenKind::ASSIGN, String::from("=")),
            Token::new(TokenKind::FUNCTION, String::from("fn")),
            Token::new(TokenKind::LPAREN, String::from("(")),
            Token::new(TokenKind::IDENTIFIER, String::from("x")),
            Token::new(TokenKind::COMMA, String::from(",")),
            Token::new(TokenKind::IDENTIFIER, String::from("y")),
            Token::new(TokenKind::RPAREN, String::from(")")),
            Token::new(TokenKind::LBRACE, String::from("{")),
            Token::new(TokenKind::IDENTIFIER, String::from("x")),
            Token::new(TokenKind::PLUS, String::from("+")),
            Token::new(TokenKind::IDENTIFIER, String::from("y")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::RBRACE, String::from("}")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::LET, String::from("let")),
            Token::new(TokenKind::IDENTIFIER, String::from("result")),
            Token::new(TokenKind::ASSIGN, String::from("=")),
            Token::new(TokenKind::IDENTIFIER, String::from("add")),
            Token::new(TokenKind::LPAREN, String::from("(")),
            Token::new(TokenKind::IDENTIFIER, String::from("five")),
            Token::new(TokenKind::COMMA, String::from(",")),
            Token::new(TokenKind::IDENTIFIER, String::from("ten")),
            Token::new(TokenKind::RPAREN, String::from(")")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::BANG, String::from("!")),
            Token::new(TokenKind::MINUS, String::from("-")),
            Token::new(TokenKind::SLASH, String::from("/")),
            Token::new(TokenKind::ASTERISK, String::from("*")),
            Token::new(TokenKind::INTEGER, String::from("5")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::INTEGER, String::from("5")),
            Token::new(TokenKind::LT, String::from("<")),
            Token::new(TokenKind::INTEGER, String::from("10")),
            Token::new(TokenKind::GT, String::from(">")),
            Token::new(TokenKind::INTEGER, String::from("5")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::IF, String::from("if")),
            Token::new(TokenKind::LPAREN, String::from("(")),
            Token::new(TokenKind::INTEGER, String::from("5")),
            Token::new(TokenKind::LT, String::from("<")),
            Token::new(TokenKind::INTEGER, String::from("10")),
            Token::new(TokenKind::RPAREN, String::from(")")),
            Token::new(TokenKind::LBRACE, String::from("{")),
            Token::new(TokenKind::RETURN, String::from("return")),
            Token::new(TokenKind::TRUE, String::from("true")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::RBRACE, String::from("}")),
            Token::new(TokenKind::ELSE, String::from("else")),
            Token::new(TokenKind::LBRACE, String::from("{")),
            Token::new(TokenKind::RETURN, String::from("return")),
            Token::new(TokenKind::FALSE, String::from("false")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::RBRACE, String::from("}")),
            Token::new(TokenKind::INTEGER, String::from("10")),
            Token::new(TokenKind::EQ, String::from("==")),
            Token::new(TokenKind::INTEGER, String::from("10")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
            Token::new(TokenKind::INTEGER, String::from("10")),
            Token::new(TokenKind::NOT_EQ, String::from("!=")),
            Token::new(TokenKind::INTEGER, String::from("9")),
            Token::new(TokenKind::SEMICOLON, String::from(";")),
        ];
        assert_eq!(tokens, expected);
    }
}
