use crate::token::Token;


mod token;
mod lexer;

fn main() {
    let input = "if true { var x = 5 } else { var y = 5 }";
    let mut tokens = lexer::new_lexer(input);

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
