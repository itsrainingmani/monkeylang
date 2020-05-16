use donkey::{lexer::Lexer, token};

#[test]
fn simple_test() {
    assert_eq!(token::ILLEGAL, "ILLEGAL");
}

#[test]
fn test_next_token() {
    let mut lex = Lexer::new(String::from("=+(){},;"));

    let expected_tokens = vec![
        token::Token {
            kind: token::ASSIGN.to_string(),
            literal: "=".to_string(),
        },
        token::Token {
            kind: token::PLUS.to_string(),
            literal: "+".to_string(),
        },
        token::Token {
            kind: token::LPAREN.to_string(),
            literal: "(".to_string(),
        },
        token::Token {
            kind: token::RPAREN.to_string(),
            literal: ")".to_string(),
        },
        token::Token {
            kind: token::LBRACE.to_string(),
            literal: "{".to_string(),
        },
        token::Token {
            kind: token::RBRACE.to_string(),
            literal: "}".to_string(),
        },
        token::Token {
            kind: token::COMMA.to_string(),
            literal: ",".to_string(),
        },
        token::Token {
            kind: token::SEMICOLON.to_string(),
            literal: ";".to_string(),
        },
        // token::Token {
        //     kind: token::EOF.to_string(),
        //     literal: "".to_string(),
        // },
    ];

    let mut actual_tokens: Vec<token::Token> = Vec::new();

    for i in 0..lex.input.len() {
        let tok: token::Token = lex.next_token();
        actual_tokens.push(tok);
    }

    assert_eq!(expected_tokens, actual_tokens);
}
