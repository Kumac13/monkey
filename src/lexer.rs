use crate::token::{Token, TokenKind};

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
                    let mut num = String::from(c);
                    while let Some(nc) = input.peek() {
                        if nc.is_ascii_digit() {
                            num.push(*nc);
                            input.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::new(TokenKind::INTEGER, num));
                } else if is_literal(c) {
                    let mut literal = String::from(c);
                    while let Some(nc) = input.peek() {
                        if is_literal(*nc) {
                            literal.push(*nc);
                            input.next();
                        } else {
                            break;
                        }
                    }
                    match literal.as_str() {
                        "fn" => tokens.push(Token::new(TokenKind::FUNCTION, literal)),
                        "let" => tokens.push(Token::new(TokenKind::LET, literal)),
                        "true" => tokens.push(Token::new(TokenKind::TRUE, literal)),
                        "false" => tokens.push(Token::new(TokenKind::FALSE, literal)),
                        "if" => tokens.push(Token::new(TokenKind::IF, literal)),
                        "else" => tokens.push(Token::new(TokenKind::ELSE, literal)),
                        "return" => tokens.push(Token::new(TokenKind::RETURN, literal)),
                        _ => tokens.push(Token::new(TokenKind::IDENTIFIER, literal)),
                    }
                } else {
                    tokens.push(Token::new(TokenKind::ILLEGAL, String::from(c)));
                }
            }
        }
    }
    tokens.push(Token::new(TokenKind::EOF, String::from("")));
    tokens
}

fn is_literal(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || c.is_ascii_digit()
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
            Token::new(TokenKind::EOF, String::from("")),
        ];
        assert_eq!(tokens, expected);
    }
}
