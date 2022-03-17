use crate::token::{self, Token};
use std::default;

#[derive(Debug, Default)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

pub fn new_lexer(input: &str) -> Lexer {
    let mut lexer = Lexer {
        input: input.chars().collect::<Vec<char>>(),
        ..Default::default()
    };

    lexer.read_char();

    lexer
}

impl Lexer {
    fn read_char(&mut self) {
        // if the input current position goes out of bounds, the current char becomes 0
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            // if we find a character in the current position, assign the char with the current input character
            self.ch = self.input[self.read_position];
        }

        self.position = self.read_position;

        self.read_position += 1;
    }

    // Return the next token that match with the current character
    pub fn next_token(&mut self) -> Token {
        let token: Token;

        self.skip_whitespace();

        match self.ch {
            '"' => {
                let str_value = self.read_string();
                token = Token::String(str_value);
            }
            '=' => {
                if self.peek_char() == '=' {
                    token = Token::Eq;
                } else {
                    token = Token::Assign;
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    token = Token::NotEq;
                } else {
                    token = Token::Bang;
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    token = Token::Lte;
                }else{
                    token = Token::Lt;
                }
            },
            '>' => {
                if self.peek_char() == '=' {
                    token = Token::Gte;
                }else{
                    token = Token::Gt;
                }
            },
            ':' => token = Token::Colon,
            ',' => token = Token::Comma,
            '{' => token = Token::LBrace,
            '}' => token = Token::RBrace,
            '(' => token = Token::LParen,
            ')' => token = Token::RParen,
            '+' => token = Token::Plus,
            '-' => token = Token::Minus,
            ']' => token = Token::RBracket,
            '[' => token = Token::LBracket,
            ';' => token = Token::Semicolon,
            '*' => token = Token::Asterisk,
            '/' => token = Token::Slash,
            '\0' => token = Token::EOF,
            c => {
                if c.is_alphabetic(){
                    let identifier = self.read_identifier();

                    return match identifier.as_str(){
                        "var" => Token::Var,
                        "function" => Token::Function,
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        "if" => Token::If,
                        "else" => Token::Else,
                        "return" => Token::Return,
                        _ => Token::Ident(identifier),
                    };
                }else if c.is_digit(10) {
                    let number = self.read_number();
                    return Token::Integer(number);
                }else{
                    token = Token::Illegal;
                }
            }
        }

        self.read_char();

        token
    }

    // Peek the next char, if it's the end of the string then returns '\0'
    fn peek_char(&mut self) -> char {
        if self.read_position > self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn read_string(&mut self) -> String {
        // skip the first "
        let initial_position = self.position + 1;

        // This loop will read the string until found an EOF or the end of the string denoted by "
        loop {
            self.read_char();

            if self.ch == '\0' || self.ch == '"' {
                break;
            }
        }

        let final_position = self.position;

        self.input[initial_position..final_position]
            .iter()
            .collect::<String>()
    }

    fn read_identifier(&mut self) -> String {
        let initial_position = self.position;

        while self.ch.is_alphabetic(){
            self.read_char();
        };

        let final_position = self.position;

        self.input[initial_position..final_position]
        .iter()
        .collect::<String>()
    }


    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace(){
            self.read_char()
        }
    }



    fn read_number(&mut self) -> i32 {
        let initial_position = self.position;

        while self.ch.is_numeric() {
            self.read_char();
        }

        let final_position = self.position;

        self.input[initial_position..final_position]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .expect("Error while reading number, found char")
    }
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_tokenize_var_declaration() {
        let input = "var test = 5;";
        let mut tokens = new_lexer(input);

        let expected = vec![
            Token::Var,
            Token::Ident("test".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::EOF,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

    #[test]
    fn test_tokenize_if_else_statement() {
        let input = "if true { var x = 5 } else { var y = 5 }";
        let mut tokens = new_lexer(input);

        let expected = vec![
            Token::If,
            Token::Boolean(true),
            Token::LBrace,
            Token::Var,
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Var,
            Token::Ident("y".to_string()),
            Token::Assign,
            Token::Integer(5),
            Token::RBrace,
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

    #[test]
    fn test_tokenize_numeric_expression() {
        let input = "var myVar = 10 * 300 + 500; \
                          return myVar";
        let mut tokens = new_lexer(input);

        let expected = vec![
            Token::Var,
            Token::Ident("myVar".to_string()),
            Token::Assign,
            Token::Integer(10),
            Token::Asterisk,
            Token::Integer(300),
            Token::Plus,
            Token::Integer(500),
            Token::Semicolon,
            Token::Return,
            Token::Ident("myVar".to_string()),
        ];

        for token_expected in expected.iter() {
            let token = tokens.next_token();
            assert_eq!(&token, token_expected);
        }
    }

}