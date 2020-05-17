use donkey::{lexer::Lexer, token, token::TokenType};

#[test]
fn test_next_token() {
    let mut lex = Lexer::new(String::from("=+(){},;"));

    let expected_tokens = vec![
        token::Token {
            kind: TokenType::ASSIGN,
            literal: "=".to_string(),
        },
        token::Token {
            kind: TokenType::PLUS,
            literal: "+".to_string(),
        },
        token::Token {
            kind: TokenType::LPAREN,
            literal: "(".to_string(),
        },
        token::Token {
            kind: TokenType::RPAREN,
            literal: ")".to_string(),
        },
        token::Token {
            kind: TokenType::LBRACE,
            literal: "{".to_string(),
        },
        token::Token {
            kind: TokenType::RBRACE,
            literal: "}".to_string(),
        },
        token::Token {
            kind: TokenType::COMMA,
            literal: ",".to_string(),
        },
        token::Token {
            kind: TokenType::SEMICOLON,
            literal: ";".to_string(),
        },
        // token::Token {
        //     kind: token::EOF.to_string(),
        //     literal: "".to_string(),
        // },
    ];

    let mut actual_tokens: Vec<token::Token> = Vec::new();

    for _ in 0..lex.input.len() {
        let tok: token::Token = lex.next_token();
        actual_tokens.push(tok);
    }

    assert_eq!(expected_tokens, actual_tokens);
}
